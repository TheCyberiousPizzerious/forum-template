mod controllers;
mod models;
mod mongo_repo;
mod utilities;

use crate::controllers::user_controller;
use crate::mongo_repo::mongo_connection::establish_connection;
use crate::utilities::{api_availebility::is_api_reachable, select_collection::get_collection};

use mongodb::{Client, Collection, bson::Document};
use actix_web::{App, HttpServer, web::{Data, scope}};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
   let client: Client = establish_connection().await;
   //let client: Data<Arc<Client>> = Data::new(Arc::new(client));
   let user_storage_collection: Data<Arc<Collection<Document>>> = Data::new(Arc::new(get_collection(client, String::from("userStorage"), String::from("users"))));

   HttpServer::new(move || {
      App::new()
         .app_data(user_storage_collection.clone())
         .service(
            scope("/test")
               .service(is_api_reachable)
         .service(
            scope("/api")
              // .service("")
         )
            // api
               //requestData
               //requestLogs
               //userHandeler
               //utilityHandeler
         )
   })
   .bind("127.0.0.1:7175")?
   .run()
   .await
}