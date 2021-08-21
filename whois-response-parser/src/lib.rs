use chrono::{DateTime, Utc, ParseError};
use serde::{Serialize, Deserialize};

mod afilias;
mod afnic;

pub use afilias::parse_afilias_response;
pub use afnic::parse_afnic_response;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhoisData {
    pub domain_name: String,
    pub status: String,
    pub registrar: String,
    pub registrant: String,
    pub creation_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub last_update_date: DateTime<Utc>,
}

#[derive(Debug)]
pub enum WhoisError {
    TooManyRequests,
    ElementNotFound(&'static str),
    DateTimeParseError(ParseError),
    UnsupportedTldn,
}

impl From<ParseError> for WhoisError {
    fn from(e: ParseError) -> Self {
        WhoisError::DateTimeParseError(e)
    }
}

fn extract_string_value(line: &str) -> String {
    let value = line.split_once(":").unwrap().1;
    String::from(value.trim())
}

fn extract_datetime_value(line: &str) -> Result<DateTime<Utc>, ParseError> {
    let value = extract_string_value(line);
    value.parse::<DateTime<Utc>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn it_parses_string() {
        let s = extract_string_value("domain:      em2022.fr");
        assert_eq!(s, "em2022.fr");
    }

    #[test]
    fn it_parses_datetime() {
        let datetime = extract_datetime_value("created:     2020-02-14T15:00:47Z")
            .unwrap();
        let expected_datetime: DateTime<Utc> = DateTime::from_str("2020-02-14T15:00:47Z")
            .unwrap();
        assert_eq!(datetime, expected_datetime);
    }
}
