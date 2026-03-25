use axum::{
    Json, Router, extract::{Path, State}, response::Redirect, routing::get
};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{DateTime, Utc};
mod config;
use config::Config;
use sqlx::{PgPool, postgres::PgPoolOptions};
mod errors;
use errors::AppError;


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


#[tokio::main]
async fn main() -> Result<()> {

    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let app = Router::new()
        .route("/api/v1/health", get(check_health))
        .route("/api/v1/affiliates", get(get_all).post(create_affiliate))
        .route("/go/{code}", get(track_clicks))

        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

    tracing::info!("v{VERSION} on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn check_health() -> Json<Health> {
    tracing::info!("health hit");
    Json(Health {
        status: "okay".into(),
        version: VERSION.into(),
    })
}

async fn create_affiliate(State(pool): State<PgPool>, Json(body): Json<CreateAffiliate>,) -> Result<Json<Affiliate>, AppError> {
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
    .fetch_one(&pool)
    .await?;
    
    Ok(Json(create))
}

pub async fn get_all(State(pool): State<PgPool>,) -> Result<Json<Vec<Affiliate>>, AppError> {
    tracing::info!("get_all hit");
    let affiliates = sqlx::query_as!(
        Affiliate,
        "SELECT id, name, code, clicks, created_at, destination_url FROM affiliates"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(affiliates))
}

async fn track_clicks(State(pool): State<PgPool>, Path(code): Path<String>,) -> Result<Redirect, AppError> {
    tracing::info!("track clicks hit");
    let affiliate = sqlx::query!(
        "UPDATE affiliates 
        SET clicks = clicks + 1 
        WHERE code = $1 
        RETURNING destination_url",
        code
    )
    .fetch_optional(&pool)
    .await?;

    match affiliate {
        Some(row) => Ok(Redirect::temporary(&row.destination_url)),
        None => Err(AppError(anyhow::anyhow!("affiliate code not found"))),
    }
}