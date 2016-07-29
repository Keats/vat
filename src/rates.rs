use std::collections::HashMap;

use errors::{VatResult, VatError};

lazy_static! {
    // very basic and only contains the main rate
    static ref VAT_RATES: HashMap<String, u8> = {
        let mut m = HashMap::new();
        // Austria
        m.insert("AT".to_string(), 20);
        // Belgium
        m.insert("BE".to_string(), 21);
        // Bulgaria
        m.insert("BG".to_string(), 20);
        // Croatia
        m.insert("HR".to_string(), 25);
        // Cyprus
        m.insert("CY".to_string(), 19);
        // Czech Republic
        m.insert("CZ".to_string(), 21);
        // Denmark
        m.insert("DK".to_string(), 25);
        // Estonia
        m.insert("EE".to_string(), 20);
        // Finland
        m.insert("FI".to_string(), 24);
        // France
        m.insert("FR".to_string(), 20);
        // Germany
        m.insert("DE".to_string(), 19);
        // Greece
        m.insert("EL".to_string(), 24);
        // Hungary
        m.insert("HU".to_string(), 27);
        // Ireland
        m.insert("IE".to_string(), 23);
        // Italy
        m.insert("IT".to_string(), 22);
        // Latvia
        m.insert("LV".to_string(), 21);
        // Lithuania
        m.insert("LT".to_string(), 21);
        // Luxembourg
        m.insert("LU".to_string(), 17);
        // Malta
        m.insert("MT".to_string(), 18);
        // The Netherlands
        m.insert("NL".to_string(), 21);
        // Poland
        m.insert("PL".to_string(), 23);
        // Portugal
        m.insert("PT".to_string(), 23);
        // Romania
        m.insert("RO".to_string(), 20);
        // Slovak Republic
        m.insert("SK".to_string(), 20);
        // Slovenia
        m.insert("SI".to_string(), 22);
        // Spain
        m.insert("ES".to_string(), 21);
        // Sweden
        m.insert("SE".to_string(), 25);
        // United Kingdom
        m.insert("GB".to_string(), 20);

        m
    };
}

pub fn get_rate(country: &str) -> VatResult<u8> {
    match VAT_RATES.get(&country.to_uppercase()) {
        Some(r) => Ok(*r),
        None => Err(VatError::InvalidCountry(country.to_string()))
    }
}

pub fn get_all_rates() -> HashMap<String, u8> {
    VAT_RATES.clone()
}

#[cfg(test)]
mod tests {
    use super::get_rate;

    #[test]
    fn test_can_get_rate_eu_country() {
        let res = get_rate("fr");
        assert!(res.is_ok());
        assert_eq!(20, res.unwrap());
    }

    #[test]
    fn test_cannot_get_rate_non_eu_country() {
        let res = get_rate("us");
        assert!(res.is_err());
    }

}
