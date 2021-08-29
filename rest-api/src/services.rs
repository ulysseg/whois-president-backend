use actix_web::{get, web, HttpResponse, Responder};
use futures_util::stream::TryStreamExt;
use mongo_model::PotentialCandidate;
use mongodb::bson::{doc, Document};
use mongodb::Database;

#[get("/parties")]
pub async fn get_parties(db: web::Data<Database>) -> impl Responder {
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
        },
        doc! {
            "$sort": { "party": 1 }
        }
    ];

    let x = coll.aggregate(pipeline, None).await.unwrap();
    let x: Vec<Document> = x.try_collect().await.unwrap();

    HttpResponse::Ok().json(x)
}