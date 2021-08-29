use futures_util::stream::TryStreamExt;
use mongodb::bson;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection};
use regex::{Regex, Replacer};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;

// TODO initials if long enough or 3 words, ex nkm2022.fr

const DN_SUFFIX: &str = "2022.fr";


fn generate_domain_names(first_name: &str, last_name: &str) -> HashSet<String> {
    let first_name = normalize_str(&first_name);
    let last_name = normalize_str(&last_name);
    let full_name = format!("{} {}", first_name, last_name);

    let mut h = HashSet::new();
    // Last name only
    h.insert(remove_spaces(&remove_dashes(&last_name)) + DN_SUFFIX);
    h.insert(replace_spaces_with_dashes(&last_name) + DN_SUFFIX);
    // Full name
    h.insert(remove_spaces(&remove_dashes(&full_name)) + DN_SUFFIX);
    h.insert(replace_spaces_with_dashes(&full_name) + DN_SUFFIX);
    h
}

fn normalize_str(s: &str) -> String {
    unidecode::unidecode(s).to_lowercase()
}

fn replace_spaces<R: Replacer>(s: &str, rep: R) -> String {
    let r = Regex::new(r"\s+").unwrap();
    let r = r.replace_all(s, rep);
    r.to_string()
}

fn remove_spaces(s: &str) -> String {
    replace_spaces(s, "")
}

fn replace_spaces_with_dashes(s: &str) -> String {
    replace_spaces(s, "-")
}

fn remove_dashes(s: &str) -> String {
    s.replace("-", "")
}

// TODO use mongo-model

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PotentialCandidateDocument {
    #[serde(rename = "_id")]
    id: ObjectId,
    first_name: String,
    last_name: String,
    domain_names: Vec<DomainName>,
}

impl PotentialCandidateDocument {

    fn has_domain_name(&self, name: &str) -> bool {
        self.domain_names.iter().any(|dn| dn.name == name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DomainName {
    name: String,
}

impl DomainName {

    fn new(name: String) -> DomainName {
        DomainName {
            name,
        }
    }

}

async fn get_collection() -> Collection<PotentialCandidateDocument> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let database = client.database("whoisPresident");
    database.collection("potentialCandidates")
}

pub async fn update_candidates_with_generated_domains() -> Result<(), Box<dyn Error>> {
    let coll = get_collection().await;
    // To find candidates without an official domain name
    let filter = doc! { "domainNames.official": { "$ne": true } };
    let mut c = coll.find(filter, None).await.unwrap();
    while let Some(candidate) = c.try_next().await.unwrap() {
        println!("{:?}", candidate);
        let generated_domain_names = generate_domain_names(&candidate.first_name, &candidate.last_name);
        let x: Vec<DomainName> = generated_domain_names.into_iter()
            .filter(|dn| !candidate.has_domain_name(dn))
            .map(|dn| DomainName::new(dn))
            .collect();
        if !x.is_empty() {
            let query = doc! { "_id": candidate.id };
            let x = bson::to_bson(&x).unwrap();
            let update = doc! { "$push": { "domainNames": { "$each": x } }};
            coll.update_one(query, update, None).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_normalize_str() {
        assert_eq!(normalize_str("Acigné"), "acigne");
    }

    #[test]
    fn test_remove_spaces() {
        assert_eq!(remove_spaces("hel \t lo"), "hello");
    }

    #[test]
    fn test_replace_spaces_with_dashes() {
        assert_eq!(replace_spaces_with_dashes("hel \t lo"), "hel-lo");
    }

    #[test]
    fn test_remove_dashes() {
        assert_eq!(remove_dashes("-h-i-"), "hi");
    }

    #[test]
    fn test_generate_domain_names() {
        let dn = generate_domain_names("Jean-Yves", "Le Drian");
        assert_eq!(dn.len(), 4);
        assert!(dn.contains("ledrian2022.fr"));
        assert!(dn.contains("le-drian2022.fr"));
        assert!(dn.contains("jeanyvesledrian2022.fr"));
        assert!(dn.contains("jean-yves-le-drian2022.fr"));
    }
}