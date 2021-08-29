mod fib;

use futures_util::stream::TryStreamExt;
use mongodb::{Client, Collection, Cursor};
use mongodb::bson;
use mongodb::bson::{doc, oid::ObjectId};
use mongo_model::{PotentialCandidate, WhoisLookup};
use std::{env, thread, time};
use whois_rust::{WhoIs, WhoIsLookupOptions};
use crate::fib::FibonacciSequence;
use whois_response_parser::{WhoisData, WhoisError};

pub async fn lookup_candidates_domains() {
    let persons_collection = get_candidates_collection().await;
    let mut candidates_to_lookup = find_candidates_to_lookup(&persons_collection).await;
    let whois = create_whois();
    let mut fib = FibonacciSequence::new();

    while let Some(candidate) = candidates_to_lookup.try_next().await.unwrap() {
        lookup_candidate_domains(&whois, &mut fib, candidate, &persons_collection).await;
    }
}

async fn get_candidates_collection() -> Collection<PotentialCandidate> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let database = client.database("whoisPresident");
    log::debug!("Connected to database!");
    database.collection("potentialCandidates")
}

/// Find candidates with domains names without whois data
async fn find_candidates_to_lookup(persons_collection: &Collection<PotentialCandidate>) -> Cursor<PotentialCandidate> {
    let filter = doc! {
        "domainNames.whoisLookup.whoisData": "null",
        "enableLookup": { "$ne": false }
    };
    persons_collection.find(filter, None).await.unwrap()
}

fn create_whois() -> WhoIs {
    let whois_servers = env::var("WHOIS_SERVERS_FILE").unwrap_or(String::from("servers.json"));
    WhoIs::from_path(whois_servers).unwrap()
}

async fn lookup_candidate_domains(whois: &WhoIs, fib: &mut FibonacciSequence, candidate: PotentialCandidate,
                                  candidates_collection: &Collection<PotentialCandidate>) {
    for domain_name in candidate.domain_names {
        if domain_name.has_whois_data() {
            loop {
                // Start by sleeping because of the break below
                log::debug!("Waiting {} seconds...", fib.current());
                thread::sleep(time::Duration::from_secs(fib.current()));
                match lookup_domain_name(&whois, &domain_name.name) {
                    Ok(entries) => {
                        log::info!("Response OK");
                        update_candidate_with_lookup(candidate.id, &domain_name.name, entries,
                                                     candidates_collection).await;
                        fib.previous();
                        break;
                    },
                    Err(WhoisError::TooManyRequests) => {
                        log::warn!("Too many requests, slowing down...");
                        fib.next();
                    },
                    Err(e) => {
                        log::error!("Could not lookup {} : {:?}", &domain_name.name, e);
                        break;
                    }
                }
            }
        }
    }
}

fn lookup_domain_name(whois: &WhoIs, domain_name: &str) -> Result<Option<WhoisData>, WhoisError> {
    log::info!("Looking up domain: {:?}", domain_name);

    let lookup_options = WhoIsLookupOptions::from_str(domain_name).unwrap();
    let text_response = whois.lookup(lookup_options).unwrap();
    log::trace!("Text: {}", text_response);
    if domain_name.ends_with(".fr") {
      whois_response_parser::parse_afnic_response(&text_response)
    } else if domain_name.ends_with(".info") {
        whois_response_parser::parse_afilias_response(&text_response)
    } else {
        return Err(WhoisError::UnsupportedTldn);
    }
}

async fn update_candidate_with_lookup(person_id: ObjectId, domain_name: &str, whois_data: Option<WhoisData>,
                                      persons_collection: &Collection<PotentialCandidate>) {
    let query = doc! {"_id": person_id, "domainNames.name": domain_name };
    let whois_lookup = WhoisLookup::new(whois_data);
    let whois_lookup = bson::to_bson(&whois_lookup).unwrap();
    let update = doc! { "$set": { "domainNames.$.whoisLookup": whois_lookup } };
    persons_collection.update_one(query, update, None).await.unwrap();
}
