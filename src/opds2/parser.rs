//! OPDS 2.0 Parser

use std::io::Read;

use crate::common::error::Result;
use crate::opds2::{Feed, Publication};

/// OPDS 2.0 Parser
pub struct Parser;

impl Parser {
    /// Create a new OPDS 2.0 parser
    pub fn new() -> Self {
        Self
    }

    /// Parse OPDS 2.0 feed from JSON string
    pub fn parse_feed(&self, json: &str) -> Result<Feed> {
        serde_json::from_str(json).map_err(|e| {
            crate::common::error::OpdsError::Custom(format!("Failed to parse OPDS 2.0 feed: {}", e))
        })
    }

    /// Parse OPDS 2.0 feed from reader
    pub fn parse_feed_from_reader<R: Read>(&self, mut reader: R) -> Result<Feed> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        self.parse_feed(&buffer)
    }

    /// Parse OPDS 2.0 publication from JSON string
    pub fn parse_publication(&self, json: &str) -> Result<Publication> {
        serde_json::from_str(json).map_err(|e| {
            crate::common::error::OpdsError::Custom(format!(
                "Failed to parse OPDS 2.0 publication: {}",
                e
            ))
        })
    }

    /// Validate a parsed feed
    pub fn validate_feed(&self, feed: &Feed) -> crate::common::error::ValidationResult {
        feed.validate()
    }

    /// Validate a parsed publication
    pub fn validate_publication(
        &self,
        pub_: &Publication,
    ) -> crate::common::error::ValidationResult {
        pub_.validate()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_new() {
        let parser = Parser::new();
        assert!(matches!(parser, Parser));
    }

    #[test]
    fn test_validate_feed() {
        let parser = Parser::new();
        use crate::opds2::{Feed, Link};
        // Create a valid feed with at least one collection (publications)
        let publications = vec![];
        let feed = Feed::new("Test Feed", vec![Link::new("self", "http://example.com")])
            .with_publications(publications);

        assert!(parser.validate_feed(&feed).is_ok());
    }

    #[test]
    fn test_validate_publication() {
        let parser = Parser::new();
        use crate::opds2::{Link, Publication, PublicationMetadata};
        // Create a valid publication with acquisition link
        let publication = Publication::new(
            PublicationMetadata::new("Test Publication"),
            vec![
                Link::new("self", "http://example.com"),
                Link::new("download", "http://example.com/book.epub"),
            ],
        );

        assert!(parser.validate_publication(&publication).is_ok());
    }
}
