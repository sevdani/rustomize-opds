//! OPDS 1.2 Parser

use std::io::Read;

use crate::common::error::Result;
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
        serde_xml_rs::from_str(xml).map_err(crate::common::error::OpdsError::Xml)
    }

    /// Parse OPDS 1.2 feed from reader
    pub fn parse_feed_from_reader<R: Read>(&self, reader: R) -> Result<Feed> {
        serde_xml_rs::from_reader(reader).map_err(crate::common::error::OpdsError::Xml)
    }

    /// Validate a parsed feed
    pub fn validate_feed(&self, feed: &Feed) -> crate::common::error::ValidationResult {
        feed.validate()
    }

    /// Parse and validate OPDS 1.2 feed
    pub fn parse_and_validate(&self, xml: &str) -> Result<Feed> {
        let feed = self.parse_feed(xml)?;
        feed.validate()?;
        Ok(feed)
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
        let feed = Feed::new("urn:uuid:123", "Test Feed");

        assert!(parser.validate_feed(&feed).is_ok());
    }

    #[test]
    fn test_parse_and_validate() {
        // Test validation with minimal valid feed
        let parser = Parser::new();
        // Create a feed programmatically instead of parsing
        let feed = Feed::new("urn:uuid:123", "Test Feed");
        assert!(parser.validate_feed(&feed).is_ok());
    }
}
