use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::product_model::Product;

pub struct MongoRepo {
    col: Collection<Product>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("rustDB");
        let col: Collection<Product> = db.collection("Product");
        MongoRepo { col }
    }

    pub async fn create_product(&self, new_product: Product) -> Result<InsertOneResult, Error> {
        let new_doc = Product {
            id: None,
            name: new_product.name,
            description: new_product.description,
            price: new_product.price,
        };
        let product = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating product");

        Ok(product)
    }

    pub async fn get_product(&self, id: &String) -> Result<Product, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let product_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting product's detail");

        Ok(product_detail.unwrap())
    }

    pub async fn update_product(&self, id: &String, new_product: Product) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_product.id,
                    "name": new_product.name,
                    "description": new_product.description,
                    "price": new_product.price
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating product");
        Ok(updated_doc)
    }

    pub async fn delete_product(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let product_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting product");

        Ok(product_detail)
    }

    pub async fn get_all_products(&self) -> Result<Vec<Product>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of products");
        let mut products: Vec<Product> = Vec::new();
        while let Some(product) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            products.push(product)
        }
        Ok(products)
    }
}
