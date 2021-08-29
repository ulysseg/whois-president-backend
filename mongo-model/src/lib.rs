use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use whois_response_parser::WhoisData;

// TODO see if preferable to use getter instead of pub

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PotentialCandidate {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub domain_names: Vec<DomainName>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainName {
    pub name: String,
    pub whois_lookup: Option<WhoisLookup>,
}

impl DomainName {

    pub fn has_whois_data(&self) -> bool {
        self.whois_lookup.as_ref().and_then(|l| l.whois_data.as_ref()).is_some()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhoisLookup {
    date: DateTime<Utc>,
    whois_data: Option<WhoisData>,
}

impl WhoisLookup {

    pub fn new(whois_data: Option<WhoisData>) -> WhoisLookup {
        WhoisLookup {
            date: Utc::now(),
            whois_data,
        }
    }
}
