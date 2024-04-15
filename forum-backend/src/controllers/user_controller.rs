use crate::models::user_model::User; 

use actix_web::{delete, get, post, put, HttpResponse, web::{Data, Json}};
use mongodb::{bson::{Document, oid::ObjectId}, Collection};
use std::sync::Arc;
use uuid::Uuid;

#[post("/create_user")]
pub async fn create_user(collection: Data<Arc<Collection<Document>>>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        _id: ObjectId::new(),
        user_id: new_user.user_id.to_owned(),
        admin: new_user.admin.to_owned(),
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        passwordhash: new_user.passwordhash.to_owned(),
        timestamp: new_user.timestamp.to_owned(),
    };
    //client.database("userStorage");

    HttpResponse::Ok().json("")

}