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

// interro, tempo si nec, retry
pub async fn perform() {
    let persons_collection = get_collection().await;
    let mut domains_to_lookup = find_domains_to_lookup(&persons_collection).await;
    let whois_servers = env::var("WHOIS_SERVERS_FILE").unwrap_or(String::from("servers.json"));
    let whois = WhoIs::from_path(whois_servers).unwrap();
    let mut fib = FibonacciSequence::new();

    while let Some(candidate) = domains_to_lookup.try_next().await.unwrap() {
        process_person(&whois, &mut fib, candidate, &persons_collection).await;
    }
}

async fn get_collection() -> Collection<PotentialCandidate> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let database = client.database("whoisPresident");
    log::debug!("Connected to database!");
    database.collection("potentialCandidates")
}

/// Find candidates with domains names without whois data
async fn find_domains_to_lookup(persons_collection: &Collection<PotentialCandidate>) -> Cursor<PotentialCandidate> {
    let filter = doc! {
        "domainNames": { "$elemMatch": { "whoisLookup.entries": { "$exists": false }}},
        "enableLookup": { "$ne": false }
    };
    persons_collection.find(filter, None).await.unwrap()
}

async fn process_person(whois: &WhoIs, fib: &mut FibonacciSequence, person: PotentialCandidate,
                  persons_collection: &Collection<PotentialCandidate>) {
    for domain_name in person.domain_names {
        loop {
            // Start by sleeping because of the break below
            log::debug!("Waiting {} seconds...", fib.current());
            thread::sleep(time::Duration::from_secs(fib.current()));
            match lookup_domain_name(&whois, &domain_name.name) {
                Ok(entries) => {
                    log::info!("Response OK");
                    update_person(person.id, &domain_name.name, entries,
                                  persons_collection).await;
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

async fn update_person(person_id: ObjectId, domain_name: &str, entries: Option<WhoisData>,
                 persons_collection: &Collection<PotentialCandidate>) {
    let query = doc! {"_id": person_id, "domainNames.name": domain_name };

    let entries = WhoisLookup::new(entries);
    let e = bson::to_bson(&entries).unwrap();
    let update = doc! { "$set": { "domainNames.$.whoisLookup": e } };
    persons_collection.update_one(query, update, None).await.unwrap();
}
