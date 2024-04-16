use crate::models::user_model::User; 

use actix_web::{delete, get, post, put, HttpResponse, web::{Data, Json}};
use mongodb::{bson::{Document, oid::ObjectId, bson::doc}, Collection};
use std::sync::Arc;
use uuid::Uuid;

#[post("/register")]
pub async fn register_user(collection: Data<Arc<Collection<Document>>>, new_user: Json<User>) -> HttpResponse {


    let data = User {
        _id: ObjectId::new(),
        user_id: Uuid:new_v4(),
        admin: false,
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        passwordhash: new_user.passwordhash.to_owned(),
        server_timestamp: new_user.timestamp.to_owned(), // Server timestamp
        user_timestamp: new_user.timestamp.to_owned(),
    };
    let document: Document = data.to_json();
    //client.database("userStorage");
    collection.insert_one(document, None).await?;
    // Burde kalle på en login funksjon og logge inn for brukeren
    HttpResponse::Ok().json()
}