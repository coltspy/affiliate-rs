
#### This is a self-hosted affiliate tracking system built in Rust. You can track clicks, conversions, and revenue across your affiliate network with a single API.

## Features

- **Affiliate management** -- full CRUD with unique tracking codes and custom destination URLs
- **Click tracking** -- redirect via `/go/{code}`, logs IP, user agent, referer, and sub-ID
- **Conversion tracking** -- log events (signups, deposits, purchases) with amounts and arbitrary metadata
- **Stats and reporting** -- daily click breakdown, total conversions, revenue grouped by event type
- **Sub-ID support** -- affiliates tag campaigns with `?sub_id=x` for granular attribution
- **API key authentication** -- protect admin routes with a bearer token
- **Rate limiting** -- per-IP rate limiting on the redirect endpoint
- **CORS support** -- configurable cross-origin requests
- **Pagination** -- paginated list endpoints with configurable page size
- **Proper error handling** -- structured JSON errors with 404, 409, 422, and 500 status codes
- **Docker deployment** -- one-command setup with docker-compose

## Quick Start

```bash
git clone https://github.com/coltspy/affiliate-rs.git
cd affiliate
docker compose up
```

The API starts at `http://localhost:3000`. 

The default API key is set to `change-me-in-production` you should update it in `docker-compose.yml` before deploying.

### Test the server:

```bash
curl http://localhost:3000/api/v1/health
```

## Manual Setup

### Prerequisites

- [Rust](https://rustup.rs/) (1.86+)
- PostgreSQL 15+
- [sqlx-cli](https://crates.io/crates/sqlx-cli): `cargo install sqlx-cli --no-default-features --features postgres`

### Steps

```bash
git clone https://github.com/yourname/affiliate.git
cd affiliate/api
```

Create a `.env` file:

```
DATABASE_URL=postgres://user:password@localhost:5432/affiliate
API_KEY=your-secret-api-key
PORT=3000
```

Run migrations and start the server:

```bash
sqlx migrate run
cargo run
```

## Environment Variables

| Variable       | Required | Default | Description                        |
|----------------|----------|---------|------------------------------------|
| `DATABASE_URL` | Yes      | --      | PostgreSQL connection string       |
| `API_KEY`      | Yes      | --      | Bearer token for protected routes  |
| `PORT`         | No       | `3000`  | Port the server listens on         |

## API Reference

All protected routes require the header:

```
Authorization: Bearer <your-api-key>
```

### Health

| Method | Path               | Auth | Description          |
|--------|--------------------|------|----------------------|
| GET    | `/api/v1/health`   | No   | Health check         |

### Affiliates

| Method | Path                              | Auth | Description                          |
|--------|-----------------------------------|------|--------------------------------------|
| GET    | `/api/v1/affiliates`              | Yes  | List all affiliates (paginated)      |
| POST   | `/api/v1/affiliates`              | Yes  | Create a new affiliate               |
| GET    | `/api/v1/affiliates/{id}`         | Yes  | Get affiliate by ID                  |
| PUT    | `/api/v1/affiliates/{id}`         | Yes  | Update affiliate (partial supported) |
| DELETE | `/api/v1/affiliates/{id}`         | Yes  | Delete affiliate                     |
| GET    | `/api/v1/affiliates/{code}/stats` | Yes  | Get click and conversion stats       |

### Conversions

| Method | Path                  | Auth | Description                        |
|--------|-----------------------|------|------------------------------------|
| POST   | `/api/v1/conversions` | Yes  | Log a conversion by affiliate code |

### Tracking

| Method | Path                  | Auth | Description                                  |
|--------|-----------------------|------|--------------------------------------------- |
| GET    | `/go/{code}?sub_id=x` | No   | Track click and redirect (rate limited)      |

---

### Examples

**Create an affiliate**

```bash
curl -X POST http://localhost:3000/api/v1/affiliates \
  -H "Authorization: Bearer your-secret-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "James Wilson",
    "code": "james",
    "destination_url": "https://example.com/signup"
  }'
```

```json
{
  "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "James Wilson",
  "code": "james",
  "clicks": 0,
  "destination_url": "https://example.com/signup",
  "created_at": "2026-03-25T05:33:00Z"
}
```

**Track a click**

Visiting `http://localhost:3000/go/james?sub_id=twitter-bio` redirects to `https://example.com/signup` and logs the click with IP, user agent, referer, and sub-ID.

**Log a conversion**

```bash
curl -X POST http://localhost:3000/api/v1/conversions \
  -H "Authorization: Bearer your-secret-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "code": "james",
    "event": "deposit",
    "amount": 150.00,
    "sub_id": "twitter-bio",
    "metadata": { "currency": "USD", "method": "credit_card" }
  }'
```

```json
{
  "id": "f9e8d7c6-b5a4-3210-fedc-ba9876543210",
  "affiliate_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "event": "deposit",
  "amount": "150.00",
  "sub_id": "twitter-bio",
  "metadata": { "currency": "USD", "method": "credit_card" },
  "created_at": "2026-03-25T06:15:00Z"
}
```

**Get stats for an affiliate**

```bash
curl http://localhost:3000/api/v1/affiliates/james/stats \
  -H "Authorization: Bearer your-secret-api-key"
```

```json
{
  "affiliate_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "code": "james",
  "total_clicks": 342,
  "total_conversions": 27,
  "daily_clicks": [
    { "date": "2026-03-25", "clicks": 89 },
    { "date": "2026-03-24", "clicks": 104 },
    { "date": "2026-03-23", "clicks": 149 }
  ],
  "conversions_by_event": [
    { "event": "signup", "count": 18, "total_amount": null },
    { "event": "deposit", "count": 9, "total_amount": "4250.00" }
  ]
}
```

**List affiliates (paginated)**

```bash
curl "http://localhost:3000/api/v1/affiliates?page=1&limit=10" \
  -H "Authorization: Bearer your-secret-api-key"
```

```json
{
  "data": [
    {
      "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "name": "James Wilson",
      "code": "james",
      "clicks": 342,
      "destination_url": "https://example.com/signup",
      "created_at": "2026-03-25T05:33:00Z"
    }
  ],
  "total": 1,
  "page": 1,
  "limit": 10
}
```

## Docker (recommended setup):

The included `docker-compose.yml` runs the API and a PostgreSQL 17 database:

```yaml
services:
  db:
    image: postgres:17
    environment:
      POSTGRES_USER: affiliate
      POSTGRES_PASSWORD: affiliate
      POSTGRES_DB: affiliate
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data

  api:
    build: .
    ports:
      - "3000:3000"
    environment:
      DATABASE_URL: postgres://affiliate:affiliate@db:5432/affiliate
      API_KEY: change-me-in-production
      PORT: "3000"
    depends_on:
      - db
```

Migrations run automatically on startup. The API image uses a multi-stage build to keep the final image small (Debian slim with just the binary).

To run in production, update `API_KEY` and the database credentials, then:

```bash
docker compose up -d
```

