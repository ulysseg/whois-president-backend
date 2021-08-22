use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures_util::stream::TryStreamExt;
use mongo_model::PotentialCandidate;
use mongodb::bson::{doc, Document};
use mongodb::{Client, options::ClientOptions, Database, Collection};
use serde::{Serialize, Deserialize};
use std::env;
use futures_util::StreamExt;

#[get("/parties")]
async fn get_parties(db: web::Data<Database>) -> impl Responder {
    let coll = db.collection::<PotentialCandidate>("potentialCandidates");
    let pipeline = vec![
        doc! {
            "$sort": { "firstName": 1 }
        },
        doc! {
            "$group": {
                "_id": "$party",
                "candidates": { "$push": "$$ROOT"}
            }
        },
        doc! {
            "$project": {
                  "_id": 0,
                  "party": "$_id",
                  "candidates": "$candidates"
               }
        }
    ];

    let x = coll.aggregate(pipeline, None).await.unwrap();
    let x: Vec<Document> = x.try_collect().await.unwrap();

    HttpResponse::Ok().json(x)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await.unwrap();
    let db = client.database("whoisPresident");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(web::scope("/api/v1")
                .service(get_parties)
            )
    })
        .bind(env::var("BIND_ADDRESS").unwrap_or(String::from("127.0.0.1:8080")))?
        .run()
        .await
}
