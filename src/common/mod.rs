//! OPDS Common Module

pub mod error;
pub mod relations;
pub mod utils;

pub use error::{OpdsError, Result, Validate, ValidationResult};
pub use relations::{opds12, opds2};
pub use utils::{format_date, is_blank, is_valid_currency, is_valid_image_format, parse_date};
