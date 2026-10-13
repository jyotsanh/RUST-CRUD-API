use crate::{models::product_model::Product, repository::sqlite_repo::SqliteRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use serde::Deserialize;

const DEFAULT_LIMIT: i64 = 100;
const MAX_LIMIT: i64 = 1000;

#[derive(Deserialize)]
pub struct ListParams {
    limit: Option<i64>,
    after: Option<i64>,
}

#[post("/items")]
pub async fn create_product(db: Data<SqliteRepo>, new_product: Json<Product>) -> HttpResponse {
    let product_detail = db.create_product(new_product.into_inner()).await;

    match product_detail {
        Ok(product) => HttpResponse::Created().json(product),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/items/{id}")]
pub async fn get_product(db: Data<SqliteRepo>, path: Path<i64>) -> HttpResponse {
    let product_detail = db.get_product(path.into_inner()).await;

    match product_detail {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body("No product found with specified ID"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/items/{id}")]
pub async fn update_product(
    db: Data<SqliteRepo>,
    path: Path<i64>,
    new_product: Json<Product>,
) -> HttpResponse {
    let update_result = db.update_product(path.into_inner(), new_product.into_inner()).await;

    match update_result {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body("No product found with specified ID"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/items/{id}")]
pub async fn delete_product(db: Data<SqliteRepo>, path: Path<i64>) -> HttpResponse {
    let result = db.delete_product(path.into_inner()).await;

    match result {
        Ok(1) => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().json("Product with specified ID not found!"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/items")]
pub async fn list_products(db: Data<SqliteRepo>, params: Query<ListParams>) -> HttpResponse {
    let limit = params.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let after = params.after.unwrap_or(0);
    let products = db.list_products(after, limit).await;

    match products {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
