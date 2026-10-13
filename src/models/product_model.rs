use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    #[serde(skip_deserializing)]
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub price: f64,
}
