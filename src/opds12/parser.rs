//! OPDS 1.2 Parser

use std::io::Read;

use crate::common::error::{OpdsError, Result};
use crate::opds12::Feed;

/// OPDS 1.2 Parser
pub struct Parser;

impl Parser {
    /// Create a new OPDS 1.2 parser
    pub fn new() -> Self {
        Self
    }

    /// Parse OPDS 1.2 feed from XML string
    pub fn parse_feed(&self, xml: &str) -> Result<Feed> {
        serde_xml_rs::from_str(xml).map_err(OpdsError::Xml)
    }

    /// Parse OPDS 1.2 feed from reader
    pub fn parse_feed_from_reader<R: Read>(&self, reader: R) -> Result<Feed> {
        serde_xml_rs::from_reader(reader).map_err(OpdsError::Xml)
    }

    /// Parse OPDS 1.2 feed from XML string with auto-validation
    pub fn parse_and_validate_feed(&self, xml: &str) -> Result<Feed> {
        let feed: Feed = serde_xml_rs::from_str(xml).map_err(OpdsError::Xml)?;
        feed.validate()?;
        Ok(feed)
    }

    /// Parse OPDS 1.2 feed from reader with auto-validation
    pub fn parse_and_validate_feed_from_reader<R: Read>(&self, reader: R) -> Result<Feed> {
        let feed: Feed = serde_xml_rs::from_reader(reader).map_err(OpdsError::Xml)?;
        feed.validate()?;
        Ok(feed)
    }

    /// Validate a parsed feed
    pub fn validate_feed(&self, feed: &Feed) -> crate::common::error::ValidationResult {
        feed.validate()
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
        let parser = Parser::new();
        let xml = crate::opds12::Generator::generate_minimal_feed(
            "urn:uuid:123",
            "Test Feed",
            "http://example.com/feed.xml",
        );
        let xml_str = serde_xml_rs::to_string(&xml).unwrap();

        assert!(parser.parse_feed(&xml_str).is_ok());
    }

    #[test]
    fn test_parse_and_validate_feed() {
        let parser = Parser::new();
        let xml = crate::opds12::Generator::generate_minimal_feed(
            "urn:uuid:123",
            "Test Feed",
            "http://example.com/feed.xml",
        );
        let xml_str = serde_xml_rs::to_string(&xml).unwrap();

        assert!(parser.parse_and_validate_feed(&xml_str).is_ok());
    }

    #[test]
    fn test_validate_feed() {
        let parser = Parser::new();
        let feed = Feed::new("urn:uuid:123", "Test Feed");

        assert!(parser.validate_feed(&feed).is_ok());
    }
}
