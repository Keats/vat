extern crate vat;

use vat::{validate_format, validate_vat_number, VatError, get_rate, get_all_rates};


fn is_vat_number_valid(vat_number: &str) -> bool {
    // checks if the format is ok first
    if !validate_format(vat_number) {
        return false;
    }

    // format is valid, check that this is an existing VAT number
    match validate_vat_number(vat_number) {
        Ok(_) => true,
        Err(e) => match e {
            // could be the api is down, allow it since the format is ok
            // in practice you would probably refuse it
            VatError::HttpError(_) => true,
            _ => false,
        }
    }
}

fn main() {
    let vat_number = "LU26375245";

    if !is_vat_number_valid(vat_number) {
        panic!("INCORRECT");
    }

    // we can unwrap safely because the vat number is valid so the country
    // will have a rate
    let vat_rate = get_rate(&vat_number[0..2]).unwrap();
    println!("VAT rate: {}", vat_rate);
    println!("All VAT rates: {:?}", get_all_rates());
}
