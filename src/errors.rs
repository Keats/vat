use std::io;

use hyper;

/// Library generic result type.
pub type VatResult<T> = Result<T, VatError>;


quick_error! {
    #[derive(Debug)]
    pub enum VatError {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }

        HttpError(err: hyper::Error) {
            from()
            description("hyper error")
            display("hyper error: {}", err)
            cause(err)
        }

        InvalidVatNumber(vat_number: String) {
            display("VAT number {} is invalid", vat_number)
            description("invalid vat number")
        }

        InvalidCountry(country: String) {
            display("Code {} is not a valid EU country", country)
            description("invalid country code")
        }
    }
}
