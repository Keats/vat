#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate hyper;
#[macro_use]
extern crate quick_error;


mod format;
mod rates;
mod vies;
mod errors;


pub use format::validate_format;
pub use vies::{validate_vat_number};
pub use rates::{get_rate, get_all_rates};
pub use errors::VatError;

