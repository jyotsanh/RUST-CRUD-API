use std::env;
use std::str::FromStr;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteSynchronous},
    Error,
};

use crate::models::product_model::Product;

pub struct SqliteRepo {
    pool: SqlitePool,
}

impl SqliteRepo {
    pub async fn init() -> Self {
        let url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/products.db".to_string());
        let options = SqliteConnectOptions::from_str(&url)
            .expect("invalid DATABASE_URL")
            .create_if_missing(true)
            // WAL lets reads run while a write is in progress; NORMAL skips the
            // per-commit fsync, which is safe from corruption in WAL mode
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);
        // create_if_missing only creates the file, not its parent directory
        if let Some(dir) = options.get_filename().parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir).expect("error creating database directory");
        }
        let pool = SqlitePoolOptions::new()
            .max_connections(16)
            .connect_with(options)
            .await
            .expect("error connecting to database");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("error running migrations");
        SqliteRepo { pool }
    }

    pub async fn create_product(&self, new_product: Product) -> Result<Product, Error> {
        sqlx::query_as::<_, Product>(
            "INSERT INTO products (name, description, price) VALUES (?, ?, ?)
             RETURNING id, name, description, price",
        )
        .bind(new_product.name)
        .bind(new_product.description)
        .bind(new_product.price)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_product(&self, id: i64) -> Result<Option<Product>, Error> {
        sqlx::query_as::<_, Product>("SELECT id, name, description, price FROM products WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update_product(&self, id: i64, new_product: Product) -> Result<Option<Product>, Error> {
        sqlx::query_as::<_, Product>(
            "UPDATE products SET name = ?, description = ?, price = ? WHERE id = ?
             RETURNING id, name, description, price",
        )
        .bind(new_product.name)
        .bind(new_product.description)
        .bind(new_product.price)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete_product(&self, id: i64) -> Result<u64, Error> {
        let result = sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Keyset pagination: returns up to `limit` products with id greater than `after`.
    pub async fn list_products(&self, after: i64, limit: i64) -> Result<Vec<Product>, Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, name, description, price FROM products WHERE id > ? ORDER BY id LIMIT ?",
        )
        .bind(after)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }
}
