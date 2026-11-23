//! OPDS 2.0 Parser

use std::io::Read;

use crate::common::error::{OpdsError, Result};
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
        serde_json::from_str(json)
            .map_err(|e| OpdsError::Custom(format!("Failed to parse OPDS 2.0 feed: {}", e)))
    }

    /// Parse OPDS 2.0 feed from reader
    pub fn parse_feed_from_reader<R: Read>(&self, mut reader: R) -> Result<Feed> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        self.parse_feed(&buffer)
    }

    /// Parse OPDS 2.0 feed from JSON string with auto-validation
    pub fn parse_and_validate_feed(&self, json: &str) -> Result<Feed> {
        let feed: Feed = serde_json::from_str(json)
            .map_err(|e| OpdsError::Custom(format!("Failed to parse OPDS 2.0 feed: {}", e)))?;
        feed.validate()?;
        Ok(feed)
    }

    /// Parse OPDS 2.0 feed from reader with auto-validation
    pub fn parse_and_validate_feed_from_reader<R: Read>(&self, mut reader: R) -> Result<Feed> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        self.parse_and_validate_feed(&buffer)
    }

    /// Parse OPDS 2.0 publication from JSON string
    pub fn parse_publication(&self, json: &str) -> Result<Publication> {
        serde_json::from_str(json)
            .map_err(|e| OpdsError::Custom(format!("Failed to parse OPDS 2.0 publication: {}", e)))
    }

    /// Parse OPDS 2.0 publication from JSON string with auto-validation
    pub fn parse_and_validate_publication(&self, json: &str) -> Result<Publication> {
        let pub_: Publication = serde_json::from_str(json).map_err(|e| {
            OpdsError::Custom(format!("Failed to parse OPDS 2.0 publication: {}", e))
        })?;
        pub_.validate()?;
        Ok(pub_)
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
    fn test_parse_feed() {
        use crate::opds2::generator::LinkWithType;

        let parser = Parser::new();
        let publications = vec![];
        let self_link = crate::opds2::Link::new("self", "http://example.com/feed.json")
            .with_type("application/opds+json");
        let feed =
            crate::opds2::Feed::new("Test Feed", vec![self_link]).with_publications(publications);
        let json = serde_json::to_string_pretty(&feed).unwrap();

        assert!(parser.parse_feed(&json).is_ok());
    }

    #[test]
    fn test_parse_and_validate_feed() {
        use crate::opds2::generator::LinkWithType;

        let parser = Parser::new();
        let publications = vec![];
        let self_link = crate::opds2::Link::new("self", "http://example.com/feed.json")
            .with_type("application/opds+json");
        let feed =
            crate::opds2::Feed::new("Test Feed", vec![self_link]).with_publications(publications);
        let json = serde_json::to_string_pretty(&feed).unwrap();

        assert!(parser.parse_and_validate_feed(&json).is_ok());
    }

    #[test]
    fn test_parse_publication() {
        let parser = Parser::new();
        let publication = crate::opds2::Publication::new(
            crate::opds2::PublicationMetadata::new("Test Publication"),
            vec![
                crate::opds2::Link::new("self", "http://example.com"),
                crate::opds2::Link::new("download", "http://example.com/book.epub"),
            ],
        );
        let json = serde_json::to_string_pretty(&publication).unwrap();

        assert!(parser.parse_publication(&json).is_ok());
    }

    #[test]
    fn test_parse_and_validate_publication() {
        let parser = Parser::new();
        let publication = crate::opds2::Publication::new(
            crate::opds2::PublicationMetadata::new("Test Publication"),
            vec![
                crate::opds2::Link::new("self", "http://example.com"),
                crate::opds2::Link::new("download", "http://example.com/book.epub"),
            ],
        );
        let json = serde_json::to_string_pretty(&publication).unwrap();

        assert!(parser.parse_and_validate_publication(&json).is_ok());
    }

    #[test]
    fn test_validate_feed() {
        let parser = Parser::new();
        use crate::opds2::{Feed, Link};
        let publications = vec![];
        let feed = Feed::new("Test Feed", vec![Link::new("self", "http://example.com")])
            .with_publications(publications);

        assert!(parser.validate_feed(&feed).is_ok());
    }

    #[test]
    fn test_validate_publication() {
        let parser = Parser::new();
        use crate::opds2::{Link, Publication, PublicationMetadata};
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
