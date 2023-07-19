mod server;
mod handlers;
mod models;
mod routes;
mod json_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::start_server().await
}