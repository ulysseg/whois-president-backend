use crate::{WhoisData, WhoisError};

pub fn parse_afilias_response(response: &str) -> Result<Option<WhoisData>, WhoisError> {
    let mut lines = response.lines();
    let mut domain_name = None;
    let mut updated_date = None;
    let mut creation_date = None;
    let mut registry_expiry_date = None;
    let mut registrar = None;
    let mut domain_status = None;
    let mut registrant_organization = None;
    while let Some(line) = lines.next() {
        if line.starts_with("Domain Name:") {
            domain_name = Some(crate::extract_string_value(line));
        }
        if line.starts_with("Updated Date:") {
            updated_date = Some(crate::extract_datetime_value(line)?);
        }
        if line.starts_with("Creation Date:") {
            creation_date = Some(crate::extract_datetime_value(line)?);
        }
        if line.starts_with("Registry Expiry Date:") {
            registry_expiry_date = Some(crate::extract_datetime_value(line)?);
        }
        if line.starts_with("Registrar:") {
            registrar = Some(crate::extract_string_value(line));
        }
        if line.starts_with("Domain Status:") {
            domain_status = Some(crate::extract_string_value(line));
        }
        if line.starts_with("Registrant Organization:") {
            registrant_organization = Some(crate::extract_string_value(line));
        }
    }
    Ok(Some(WhoisData {
        domain_name: domain_name.ok_or(WhoisError::ElementNotFound("Domain Name"))?,
        status: domain_status.ok_or(WhoisError::ElementNotFound("Domain Status"))?,
        registrar: registrar.ok_or(WhoisError::ElementNotFound("Registrar"))?,
        registrant: registrant_organization.ok_or(WhoisError::ElementNotFound("Registrant Organization"))?,
        creation_date: creation_date.ok_or(WhoisError::ElementNotFound("Creation Date"))?,
        expiration_date: registry_expiry_date.ok_or(WhoisError::ElementNotFound("Registry Expiry Date"))?,
        last_update_date: updated_date.ok_or(WhoisError::ElementNotFound("Updated Date"))?,
    }))
}

#[cfg(test)]
mod tests {
    use crate::afilias::parse_afilias_response;

    #[test]
    fn it_works() {
        let response = std::fs::read_to_string("./responses/afilias/entries.txt").unwrap();
        let e = parse_afilias_response(&response).unwrap();
        // TODO asserts
        println!("{:?}", e);
    }
}