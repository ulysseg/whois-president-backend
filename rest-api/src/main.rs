use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures_util::stream::TryStreamExt;
use mongo_model::PotentialCandidate;
use mongodb::bson::{doc, Document};
use mongodb::{Client, options::ClientOptions, Database, Collection};
use serde::{Serialize, Deserialize};
use futures_util::StreamExt;

#[get("/parties")]
async fn hello(db: web::Data<Database>) -> impl Responder {
    let coll = db.collection::<PotentialCandidate>("potentialCandidates");
    // let c = coll.find(None, None).await.unwrap();
    // let x: Vec<PotentialCandidate> = c.try_collect().await.unwrap();
    //
    // let d = doc! {"hello": "world"};
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

    // db.getCollection('potentialCandidates').aggregate([
    //     { $sort: { "firstName": 1 } },
    //     { $group: {
    //         _id: "$party",
    //         candidates: {$push: "$$ROOT"}
    //     }
    //     },
    //     { $project: {
    //       _id: 0,
    //       party: "$_id",
    //       candidates: "$candidates"
    //    }}
    // }])
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await.unwrap();
    let db = client.database("whoisPresident");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(web::scope("/api/v1")
                .service(hello)
            )
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
