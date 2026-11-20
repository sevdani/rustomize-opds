//! OPDS 2.0 Module

mod feed;
mod generator;
mod parser;
mod publication;
mod types;

pub use feed::{Facet, Feed, Group};
pub use generator::{Generator, LinkWithType};
pub use parser::Parser;
pub use publication::{Author, Publication, PublicationMetadata, SeriesInfo};
pub use types::{Availability, AvailabilityState, Link, LinkProperties, Price};
