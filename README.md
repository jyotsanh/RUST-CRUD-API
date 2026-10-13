# actix-sqlite-api

REST API (Product CRUD) built with Rust, Actix web and SQLite (via sqlx).

Originally based on [Build a REST API with Rust and MongoDB - Actix web Version](https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-actix-web-version-ei1), migrated from MongoDB to SQLite.

## Configuration

Set in `.env` (all optional):

```
DATABASE_URL=sqlite://data/products.db
HOST=0.0.0.0
PORT=8080
WORKERS=4   # actix worker threads; defaults to one per physical CPU core
```

The database file and its directory are created on startup, and migrations in `migrations/` run automatically.

## Run

```
cargo run
# or
docker compose up --build
```

## Endpoints

| Method | Path            | Description        |
|--------|-----------------|--------------------|
| POST   | `/items`        | Create a product (201) |
| GET    | `/items/{id}`   | Get a product |
| PUT    | `/items/{id}`   | Update a product |
| DELETE | `/items/{id}`   | Delete a product (204) |
| GET    | `/items?limit=100&after={id}` | List products with id > `after` (default limit 100, max 1000) |

Body for create/update: `{"name": "Pen", "description": "Blue", "price": 1.5}`
