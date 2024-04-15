use actix_web::{get, HttpResponse, Responder};

/// Test the availability of the server
#[get("/ping_server")]
pub async fn is_api_reachable() -> impl Responder {
 HttpResponse::Ok().json("recieved your call!")
}