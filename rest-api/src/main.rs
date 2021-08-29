use std::env;

use actix_web::{App, HttpServer, web};
use mongodb::Client;

use rest_api::services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await.unwrap();
    let db = client.database("whoisPresident");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(web::scope("/api/v1")
                .service(services::get_parties)
            )
    })
        .bind(env::var("BIND_ADDRESS").unwrap_or(String::from("127.0.0.1:8080")))?
        .run()
        .await
}
