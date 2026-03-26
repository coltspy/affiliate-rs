use axum::{
    Json, Router, extract::{Path, Query, State}, http::{HeaderMap, StatusCode}, response::{IntoResponse, Redirect}, routing::{get, post}
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use anyhow::Result;
use chrono::{DateTime, Utc};
mod config;
use config::Config;
use sqlx::{PgPool, postgres::PgPoolOptions, migrate::Migrator};
use rust_decimal::Decimal;
mod errors;
use errors::AppError;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::cors::{CorsLayer, Any};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    api_key: String,
}


const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct Health {
    status: String,
    version: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Affiliate {
    id: String,
    name: String,
    code: String,
    clicks: i32,
    created_at: DateTime<Utc>,
    destination_url: String,
}

#[derive(Deserialize)]
struct CreateAffiliate {
    name: String,
    code: String,
    destination_url: String,

}

#[derive(Deserialize)]
struct UpdateAffiliate {
    name: Option<String>,
    code: Option<String>,
    destination_url: Option<String>,
}

#[derive(Deserialize)]
struct Pagination {
    page: Option<i64>,
    limit: Option<i64>,
}

#[derive(Deserialize)]
struct RedirectParams {
    sub_id: Option<String>,
}

#[derive(Deserialize)]
struct CreateConversion {
    code: String,
    event: String,
    amount: Option<Decimal>,
    metadata: Option<serde_json::Value>,
    sub_id: Option<String>,
}

#[derive(Serialize)]
struct Conversion {
    id: String,
    affiliate_id: String,
    event: String,
    amount: Option<Decimal>,
    metadata: Option<serde_json::Value>,
    sub_id: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(Serialize)]
struct PaginatedAffiliates {
    data: Vec<Affiliate>,
    total: i64,
    page: i64,
    limit: i64,
}

#[derive(Serialize)]
struct DailyClicks {
    date: String,
    clicks: i64,
}

#[derive(Serialize)]
struct ConversionSummary {
    event: String,
    count: i64,
    total_amount: Option<Decimal>,
}

#[derive(Serialize)]
struct AffiliateStats {
    affiliate_id: String,
    code: String,
    total_clicks: i64,
    total_conversions: i64,
    daily_clicks: Vec<DailyClicks>,
    conversions_by_event: Vec<ConversionSummary>,
}


#[tokio::main]
async fn main() -> Result<()> {

    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    Migrator::new(std::path::Path::new("./migrations")).await?.run(&pool).await?;
    tracing::info!("migrations applied");

    let state = AppState { pool, api_key: config.api_key };

    let governor_config = std::sync::Arc::new(GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(10)
        .finish()
        .unwrap());

    let redirect_route = Router::new()
        .route("/go/{code}", get(track_clicks))
        .layer(GovernorLayer { config: governor_config });

    let public_routes = Router::new()
        .route("/api/v1/health", get(check_health))
        .merge(redirect_route);

    let protected_routes = Router::new()
        .route("/api/v1/affiliates", get(get_all).post(create_affiliate))
        .route("/api/v1/affiliates/{id}", get(get_affiliate).put(update_affiliate).delete(delete_affiliate))
        .route("/api/v1/affiliates/{code}/stats", get(get_stats))
        .route("/api/v1/conversions", post(create_conversion))
        .layer(axum::middleware::from_fn_with_state(state.clone(), require_api_key));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = protected_routes.merge(public_routes).with_state(state).layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

    tracing::info!("v{VERSION} on {}", listener.local_addr()?);
    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()).await?;

    Ok(())
}

async fn check_health() -> Json<Health> {
    tracing::info!("health hit");
    Json(Health {
        status: "okay".into(),
        version: VERSION.into(),
    })
}

async fn create_affiliate(State(state): State<AppState>, Json(body): Json<CreateAffiliate>) -> Result<Json<Affiliate>, AppError> {
    tracing::info!("create hit");
    let create = sqlx::query_as!(
        Affiliate,
        "INSERT INTO affiliates (name,code,destination_url)
        VALUES ($1, $2, $3)
        RETURNING id, name, code, clicks, created_at, destination_url
        ",
        body.name,
        body.code,
        body.destination_url,
    )
    .fetch_one(&state.pool)
    .await?;
    
    Ok(Json(create))
}

async fn get_all(State(state): State<AppState>, Query(params): Query<Pagination>) -> Result<Json<PaginatedAffiliates>, AppError> {
    tracing::info!("get_all hit");
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let total = sqlx::query!("SELECT COUNT(*) as count FROM affiliates")
        .fetch_one(&state.pool)
        .await?;

    let affiliates = sqlx::query_as!(
        Affiliate,
        "SELECT id, name, code, clicks, created_at, destination_url FROM affiliates ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(PaginatedAffiliates { data: affiliates, total: total.count.unwrap_or_default(), page, limit }))
}

async fn get_affiliate(State(state): State<AppState>, Path(id): Path<String>) -> Result<Json<Affiliate>, AppError> {
    tracing::info!("get_affiliate hit");
    let affiliate = sqlx::query_as!(
        Affiliate,
        "SELECT id, name, code, clicks, created_at, destination_url FROM affiliates WHERE id = $1",
        id
    )
    .fetch_optional(&state.pool)
    .await?;

    match affiliate {
        Some(affiliate) => Ok(Json(affiliate)),
        None => Err(AppError::not_found("affiliate not found")),
    }
}

async fn update_affiliate(State(state): State<AppState>, Path(id): Path<String>, Json(body): Json<UpdateAffiliate>) -> Result<Json<Affiliate>, AppError> {
    tracing::info!("update_affiliate hit");
    let affiliate = sqlx::query_as!(
        Affiliate,
        "UPDATE affiliates SET name = COALESCE($1, name), code = COALESCE($2, code), destination_url = COALESCE($3, destination_url) WHERE id = $4 RETURNING id, name, code, clicks, created_at, destination_url",
        body.name,
        body.code,
        body.destination_url,
        id
    )
    .fetch_optional(&state.pool)
    .await?;

    match affiliate {
        Some(affiliate) => Ok(Json(affiliate)),
        None => Err(AppError::not_found("affiliate not found")),
    }
}

async fn delete_affiliate(State(state): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    tracing::info!("delete_affiliate hit");
    let affiliate = sqlx::query!(
        "DELETE FROM affiliates WHERE id = $1 RETURNING id",
        id
    )
    .fetch_optional(&state.pool)
    .await?;

    match affiliate {
        Some(_) => Ok(Json(json!({"deleted": true}))),
        None => Err(AppError::not_found("affiliate not found")),
    }
}

async fn get_stats(State(state): State<AppState>, Path(code): Path<String>) -> Result<Json<AffiliateStats>, AppError> {
    let affiliate = sqlx::query!("SELECT id, code FROM affiliates WHERE code = $1", code)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::not_found("affiliate not found"))?;

    let daily_clicks = sqlx::query!("SELECT clicked_at::date::text as date, COUNT(*) as clicks FROM click_logs WHERE affiliate_id = $1 GROUP BY clicked_at::date ORDER BY clicked_at::date DESC", affiliate.id)
        .fetch_all(&state.pool)
        .await?
        .into_iter()
        .map(|row| DailyClicks {
            date: row.date.unwrap_or_default(),
            clicks: row.clicks.unwrap_or_default(),
        })
        .collect();

    let total = sqlx::query!("SELECT COUNT(*) as total FROM click_logs WHERE affiliate_id = $1", affiliate.id)
        .fetch_one(&state.pool)
        .await?;

    let total_conversions = sqlx::query!("SELECT COUNT(*) as total FROM conversions WHERE affiliate_id = $1", affiliate.id)
        .fetch_one(&state.pool)
        .await?;

    let conversions_by_event = sqlx::query!("SELECT event, COUNT(*) as count, SUM(amount) as total_amount FROM conversions WHERE affiliate_id = $1 GROUP BY event ORDER BY count DESC", affiliate.id)
        .fetch_all(&state.pool)
        .await?
        .into_iter()
        .map(|row| ConversionSummary {
            event: row.event,
            count: row.count.unwrap_or_default(),
            total_amount: row.total_amount,
        })
        .collect();

    Ok(Json(AffiliateStats {
        affiliate_id: affiliate.id,
        code: affiliate.code,
        total_clicks: total.total.unwrap_or_default(),
        total_conversions: total_conversions.total.unwrap_or_default(),
        daily_clicks,
        conversions_by_event,
    }))
}

async fn create_conversion(State(state): State<AppState>, Json(body): Json<CreateConversion>) -> Result<Json<Conversion>, AppError> {
    tracing::info!("create_conversion hit");
    let affiliate = sqlx::query!("SELECT id FROM affiliates WHERE code = $1", body.code)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::not_found("affiliate not found"))?;

    let conversion = sqlx::query_as!(
        Conversion,
        "INSERT INTO conversions (affiliate_id, event, amount, metadata, sub_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, affiliate_id, event, amount, metadata, sub_id, created_at",
        affiliate.id,
        body.event,
        body.amount,
        body.metadata,
        body.sub_id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(conversion))
}

async fn track_clicks(State(state): State<AppState>, Path(code): Path<String>, Query(params): Query<RedirectParams>, headers: HeaderMap) -> Result<Redirect, AppError> {
    tracing::info!("track clicks hit");
    let affiliate = sqlx::query!(
        "UPDATE affiliates
        SET clicks = clicks + 1
        WHERE code = $1
        RETURNING id, destination_url",
        code
    )
    .fetch_optional(&state.pool)
    .await?;

    match affiliate {
        Some(row) => {
            let ip = headers.get("x-forwarded-for").or_else(|| headers.get("x-real-ip")).and_then(|v| v.to_str().ok()).map(|s| s.to_string());
            let user_agent = headers.get("user-agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
            let referer = headers.get("referer").and_then(|v| v.to_str().ok()).map(|s| s.to_string());

            sqlx::query!("INSERT INTO click_logs (affiliate_id, ip, user_agent, referer, sub_id) VALUES ($1, $2, $3, $4, $5)", row.id, ip, user_agent, referer, params.sub_id)
                .execute(&state.pool)
                .await?;

            Ok(Redirect::temporary(&row.destination_url))
        }
        None => Err(AppError::not_found("affiliate code not found")),
    }
}

async fn require_api_key(State(state): State<AppState>, req: axum::extract::Request, next: axum::middleware::Next) -> Result<axum::response::Response, AppError> {
    let auth_header = req.headers().get("Authorization").and_then(|v| v.to_str().ok());

    match auth_header {
        Some(header) if header == format!("Bearer {}", state.api_key) => Ok(next.run(req).await),
        _ => Ok((StatusCode::UNAUTHORIZED, Json(json!({ "error": "unauthorized" }))).into_response()),
    }
}