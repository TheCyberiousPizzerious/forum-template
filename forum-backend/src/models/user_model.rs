use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub _id: ObjectId,
    pub user_id: String,
    pub admin: bool,
    pub username: String,
    pub email: String,
    pub passwordhash: String,
    pub timestamp: String,
}