use crate::{WhoisData, WhoisError};

pub fn parse_afnic_response(response: &str) -> Result<Option<WhoisData>, WhoisError> {
    let mut lines = response.lines();
    let mut domain = None;
    let mut status = None;
    let mut registrar = None;
    let mut expiry_date = None;
    let mut creation_date = None;
    let mut last_update_date = None;
    let mut contact = None;
    while let Some(line) = lines.next() {
        if line.starts_with("%% Too many requests") {
            return Err(WhoisError::TooManyRequests);
        }
        if line.starts_with("%% No entries found") {
            return Ok(None);
        }
        if line.starts_with("domain:") {
            domain = Some(crate::extract_string_value(line));
        }
        if line.starts_with("status:") {
            status = Some(crate::extract_string_value(line));
        }
        if line.starts_with("registrar:") {
            registrar = Some(crate::extract_string_value(line));
        }
        if line.starts_with("Expiry Date:") {
            expiry_date = Some(crate::extract_datetime_value(line)?);
        }
        if line.starts_with("created:") {
            creation_date = Some(crate::extract_datetime_value(line)?);
        }
        if line.starts_with("last-update:") {
            last_update_date = Some(crate::extract_datetime_value(line)?);
        }
        if line.starts_with("contact:") {
            contact = Some(crate::extract_string_value(line));
            // Should be the last element to find
            break;
        }
    }
    Ok(Some(WhoisData {
        domain_name: domain.ok_or(WhoisError::ElementNotFound("domain"))?,
        status: status.ok_or(WhoisError::ElementNotFound("status"))?,
        registrar: registrar.ok_or(WhoisError::ElementNotFound("registrar"))?,
        registrant: contact.ok_or(WhoisError::ElementNotFound("registrant"))?,
        creation_date: creation_date.ok_or(WhoisError::ElementNotFound("created"))?,
        expiration_date: expiry_date.ok_or(WhoisError::ElementNotFound("Expiry Date"))?,
        last_update_date: last_update_date.ok_or(WhoisError::ElementNotFound("last-update"))?,
    }))
}

#[cfg(test)]
mod tests {
    use crate::afnic;

    #[test]
    fn it_works() {
        let response = std::fs::read_to_string("./responses/afnic/entries.txt").unwrap();
        let response = afnic::parse_afnic_response(&response).unwrap();
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.domain_name, "em2022.fr");
        println!("{:?}", response);
    }

    #[test]
    fn test_no_entries() {
        let response = std::fs::read_to_string("./responses/afnic/no_entries.txt").unwrap();
        let response = afnic::parse_afnic_response(&response).unwrap();
        assert!(response.is_none());
    }

    #[test]
    fn too_many_requests() {
        let response = std::fs::read_to_string("./responses/afnic/too_many_requests.txt").unwrap();
        let response = afnic::parse_afnic_response(&response);
        assert!(response.is_err());
    }
}