use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use crate::routes::configure_routes;

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(configure_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}