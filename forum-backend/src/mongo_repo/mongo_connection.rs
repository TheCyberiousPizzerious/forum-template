use dotenv::dotenv;
use mongodb::Client;
use std::env;
use url_encoder;

pub async fn establish_connection() -> Client {
    println!("ESTABLESING CONNECTION TO MONGODB SERVER");
    dotenv().ok();
    let mongouri_env = env::var("MONGOURI_VM").unwrap();
    println!("environment variable: {}", mongouri_env);
    let client: Client = Client::with_uri_str(mongouri_env).await
        .expect("Could not establish connection with mongodb");
    format!("Client information: {:?}", client);
    client
}