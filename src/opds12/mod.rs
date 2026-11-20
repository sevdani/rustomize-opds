//! OPDS 1.2 Module

mod extra;
mod generator;
mod parser;
mod types;

pub use extra::FacetGroup;
pub use generator::{FeedWithLink, Generator, LinkWithProperties};
pub use parser::Parser;
pub use types::{Author, Content, Entry, Feed, IndirectAcquisition, Link, Price, Summary};
