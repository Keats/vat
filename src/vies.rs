// Call the VIES VAT API
// The api is often down (http://ec.europa.eu/taxation_customs/vies/monitoring.html)
// so don't count on the result of the validation too much
use std::io::Read;

use regex::Regex;
use hyper::Client;

use errors::{VatResult, VatError};
use format::clean_vat_number;

// http://ec.europa.eu/taxation_customs/vies/technicalInformation.html
static API_URL: &'static str = "http://ec.europa.eu/taxation_customs/vies/services/checkVatService";
static ENVELOPE: &'static str = "
<soapenv:Envelope xmlns:soapenv=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:v1=\"http://schemas.conversesolutions.com/xsd/dmticta/v1\">
    <soapenv:Header/>
    <soapenv:Body>
      <checkVat xmlns=\"urn:ec.europa.eu:taxud:vies:services:checkVat:types\">
        <countryCode>{country}</countryCode>
        <vatNumber>{number}</vatNumber>
      </checkVat>
    </soapenv:Body>
</soapenv:Envelope>
";

#[derive(Debug)]
pub struct Company {
    pub country_code: String,
    pub vat_number: String,
    pub name: String,
    pub address: String,
}

impl Company {
    fn from_api(body: &str) -> Company {
        // #yolo
        let country_code = Regex::new(r"<countryCode>(.*?)</countryCode>").unwrap()
            .captures(body).unwrap().at(1).unwrap();
        let vat_number = Regex::new(r"<vatNumber>(.*?)</vatNumber>").unwrap()
            .captures(body).unwrap().at(1).unwrap();
        let name = Regex::new(r"<name>(.*?)</name>").unwrap()
            .captures(body).unwrap().at(1).unwrap();
        let address = Regex::new(r"<address>(?s)(.*?)(?-s)</address>").unwrap()
            .captures(body).unwrap().at(1).unwrap();

        Company {
            country_code: country_code.to_string(),
            vat_number: vat_number.to_string(),
            name: name.to_string(),
            address: address.replace("\\n", "\n").to_string(),
        }
    }
}

pub fn validate_vat_number(vat_number: &str) -> VatResult<Company> {
    let cleaned_vat = clean_vat_number(vat_number);
    let client = Client::new();

    let post_data = ENVELOPE
        .replace("{country}", &cleaned_vat[0..2])
        .replace("{number}", &cleaned_vat[2..]);
    let mut res = try!(client.post(API_URL).body(&post_data).send());

    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    let valid_re = Regex::new(r"<valid>(true|false)</valid>").unwrap();
    if valid_re.captures(&body).unwrap().at(1).unwrap() == "false" {
        return Err(VatError::InvalidVatNumber(cleaned_vat.to_string()));
    }

    Ok(Company::from_api(&body))
}


#[cfg(test)]
mod tests {
    use super::{validate_vat_number, Company};

    #[test]
    fn test_parse_xml_reply() {
        let response = r#"	<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"><soap:Body><checkVatResponse xmlns="urn:ec.europa.eu:taxud:vies:services:checkVat:types"><countryCode>LU</countryCode><vatNumber>26375245</vatNumber><requestDate>2016-07-29+02:00</requestDate><valid>true</valid><name>AMAZON EUROPE CORE S.A R.L.</name><address>5, RUE PLAETIS
L-2338  LUXEMBOURG</address></checkVatResponse></soap:Body></soap:Envelope>
"#;
        let company = Company::from_api(response);
        assert_eq!(company.name, "AMAZON EUROPE CORE S.A R.L.".to_string());
        assert_eq!(company.country_code, "LU".to_string());
        assert_eq!(company.vat_number, "26375245".to_string());
        assert_eq!(company.address, "5, RUE PLAETIS\nL-2338  LUXEMBOURG".to_string());
    }

    #[test]
    fn test_query_vies_valid() {
        // Amazon europe vat number
        let result = validate_vat_number("LU26375245");
        println!("{:?}", result);
        assert!(result.is_ok());
        let company = result.unwrap();

        assert_eq!(company.name, "AMAZON EUROPE CORE S.A R.L.".to_string());
        assert_eq!(company.country_code, "LU".to_string());
        assert_eq!(company.vat_number, "26375245".to_string());
        assert_eq!(
            company.address,
            "38, AVENUE JOHN F. KENNEDY\nL-1855  LUXEMBOURG".to_string()
        );
    }

}
