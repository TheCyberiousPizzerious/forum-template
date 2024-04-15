use mongodb::{Collection, Client, bson::Document};
use std::sync::Arc;
use actix_web::web::Data;


/// Returns a collection from the userStorage database
pub fn get_collection(client: Client, database: String, collection: String) -> Collection<Document> { 
    let db = client.database(&database);
    db.collection(&collection)
}