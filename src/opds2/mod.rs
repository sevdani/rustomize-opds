//! OPDS 2.0 Module

mod feed;
mod generator;
mod parser;
mod publication;
mod types;

pub use feed::{Facet, Feed, Group};
pub use generator::{FeedWithPublications, PublicationWithLink};
pub use generator::{Generator, LinkWithProperties, LinkWithType};
pub use parser::Parser;
pub use publication::{Author, Publication, PublicationMetadata, SeriesInfo};
pub use types::{
    AcquisitionObject, Availability, AvailabilityState, Copies, Holds, Link, LinkProperties, Price,
};
