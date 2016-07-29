# VAT

[![Build Status](https://travis-ci.org/Keats/vat.svg)](https://travis-ci.org/Keats/vat)

## Introduction
This a module to validate the format and the existence of a VAT number.
It queries the [VIES api](http://ec.europa.eu/taxation_customs/vies/vatRequest.html) which is terribly
unstable and will often have one of the country not available.

It also contains the current VAT rates of the EU countries. 
It doesn't contain any historical data or the reduced rates.

TODO: see what the api returns when country db is down

## Installation
Add the following to Cargo.toml:

```toml
vat = "0.1"
```

## Usage
### VAT number validation

```rust
use vat::{validate_format, validate_vat_number, VatError};

// vat_number should contian the country code
// it can contain spaces or dashes, those will be stripped by `vat`
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
```
In the case above we are only doing verification but the `validate_vat_number` function will return
the information for that company as the following struct:

```rust
pub struct Company {
    pub country_code: String,
    pub vat_number: String,
    pub name: String,
    pub address: String,
}
```

### Get VAT rates

```rust
use vat::{get_rate, get_all_rates};

// casing is not important
let fr_rate = get_rate("fr").unwrap();
let all_rates = get_all_rates();
```
