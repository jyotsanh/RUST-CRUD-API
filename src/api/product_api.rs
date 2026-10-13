use crate::{models::product_model::Product, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

#[post("/product")]
pub async fn create_product(db: Data<MongoRepo>, new_product: Json<Product>) -> HttpResponse {
    let data = Product {
        id: None,
        name: new_product.name.to_owned(),
        description: new_product.description.to_owned(),
        price: new_product.price,
    };

    let product_detail = db.create_product(data).await;

    match product_detail {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/product/{id}")]
pub async fn get_product(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let product_detail = db.get_product(&id).await;

    match product_detail {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/product/{id}")]
pub async fn update_product(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_product: Json<Product>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = Product {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_product.name.to_owned(),
        description: new_product.description.to_owned(),
        price: new_product.price,
    };

    let update_result = db.update_product(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_product_info = db.get_product(&id).await;

                return match updated_product_info {
                    Ok(product) => HttpResponse::Ok().json(product),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No product found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/product/{id}")]
pub async fn delete_product(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_product(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Product successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Product with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/products")]
pub async fn get_all_products(db: Data<MongoRepo>) -> HttpResponse {
    let products = db.get_all_products().await;

    match products {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
