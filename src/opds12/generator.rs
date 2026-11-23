//! OPDS 1.2 Generator

use std::io::Write;

use crate::common::error::{OpdsError, Result, ValidationResult};
use crate::opds12::{Feed, IndirectAcquisition, Link, Price};

/// OPDS 1.2 Generator
pub struct Generator;

impl Generator {
    /// Create a new OPDS 1.2 generator
    pub fn new() -> Self {
        Self
    }

    /// Generate OPDS 1.2 feed to XML string
    pub fn generate_feed(&self, feed: &Feed) -> Result<String> {
        serde_xml_rs::to_string(feed).map_err(OpdsError::Xml)
    }

    /// Generate OPDS 1.2 feed to writer
    pub fn generate_feed_to_writer<W: Write>(&self, feed: &Feed, mut writer: W) -> Result<()> {
        let xml = serde_xml_rs::to_string(feed)?;
        writer.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// Generate OPDS 1.2 feed to XML string with auto-validation
    pub fn generate_and_validate_feed(&self, feed: &Feed) -> Result<String> {
        feed.validate()?;
        serde_xml_rs::to_string(feed).map_err(OpdsError::Xml)
    }

    /// Generate OPDS 1.2 feed to writer with auto-validation
    pub fn generate_and_validate_feed_to_writer<W: Write>(&self, feed: &Feed, mut writer: W) -> Result<()> {
        feed.validate()?;
        let xml = serde_xml_rs::to_string(feed)?;
        writer.write_all(xml.as_bytes())?;
        Ok(())
    }

    /// Validate a parsed feed
    pub fn validate_feed(&self, feed: &Feed) -> ValidationResult {
        feed.validate()
    }

    /// Generate a minimal OPDS 1.2 feed with a self link
    pub fn generate_minimal_feed(
        id: impl Into<String>,
        title: impl Into<String>,
        self_href: impl Into<String>,
    ) -> Feed {
        Feed::new(id, title).with_self_link(self_href)
    }

    /// Create a new OPDS 1.2 price element
    pub fn price(value: impl Into<String>, currency: impl Into<String>) -> Price {
        Price {
            value: value.into(),
            currency_code: currency.into(),
        }
    }

    /// Create a new OPDS 1.2 indirect acquisition element
    pub fn indirect_acquisition(r#type: impl Into<String>) -> IndirectAcquisition {
        IndirectAcquisition {
            r#type: r#type.into(),
            child: None,
        }
    }

    /// Create a nested indirect acquisition chain
    pub fn nested_indirect_acquisition(
        outer_type: impl Into<String>,
        inner_type: impl Into<String>,
    ) -> IndirectAcquisition {
        IndirectAcquisition {
            r#type: outer_type.into(),
            child: Some(vec![Self::indirect_acquisition(inner_type)]),
        }
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for Feed to add links
pub trait FeedWithLink {
    /// Add a self link to the feed
    fn with_self_link(self, href: impl Into<String>) -> Self;
    /// Add a link to the feed
    fn with_link(self, link: Link) -> Self;
}

impl FeedWithLink for Feed {
    fn with_self_link(mut self, href: impl Into<String>) -> Self {
        self.links.push(
            Link::new("self", href)
                .with_type("application/atom+xml;profile=opds-catalog;kind=navigation"),
        );
        self
    }

    fn with_link(mut self, link: Link) -> Self {
        self.links.push(link);
        self
    }
}

/// Extension trait for Link to set properties
pub trait LinkWithProperties {
    /// Set the media type
    fn with_type(self, type_: impl Into<String>) -> Self;
    /// Set the title
    fn with_title(self, title: impl Into<String>) -> Self;
    /// Set the hreflang
    fn with_hreflang(self, hreflang: impl Into<String>) -> Self;
    /// Set the facet group
    fn with_facet_group(self, group: impl Into<String>) -> Self;
    /// Set as active facet
    fn into_active_facet(self) -> Self;
    /// Add a price element
    fn with_price(self, price: Price) -> Self;
    /// Add an indirect acquisition element
    fn with_indirect_acquisition(self, acquisition: IndirectAcquisition) -> Self;
}

impl LinkWithProperties for Link {
    fn with_type(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    fn with_hreflang(mut self, hreflang: impl Into<String>) -> Self {
        self.hreflang = Some(hreflang.into());
        self
    }

    fn with_facet_group(mut self, group: impl Into<String>) -> Self {
        self.opds_facet_group = Some(group.into());
        self
    }

    fn into_active_facet(mut self) -> Self {
        self.opds_active_facet = Some("true".to_string());
        self
    }

    fn with_price(mut self, price: Price) -> Self {
        if self.price.is_none() {
            self.price = Some(Vec::new());
        }
        self.price.as_mut().unwrap().push(price);
        self
    }

    fn with_indirect_acquisition(mut self, acquisition: IndirectAcquisition) -> Self {
        if self.indirect_acquisition.is_none() {
            self.indirect_acquisition = Some(Vec::new());
        }
        self.indirect_acquisition
            .as_mut()
            .unwrap()
            .push(acquisition);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opds12::{Entry, Feed, Link};

    #[test]
    fn test_generator_new() {
        let generator = Generator::new();
        assert!(matches!(generator, Generator));
    }

    #[test]
    fn test_generate_minimal_feed() {
        let feed = Generator::generate_minimal_feed(
            "urn:uuid:123",
            "Test Feed",
            "http://example.com/feed.xml",
        );

        assert_eq!(feed.id, "urn:uuid:123");
        assert_eq!(feed.title, "Test Feed");
        assert_eq!(feed.links.len(), 1);
        assert_eq!(feed.links[0].rel, "self");
    }

    #[test]
    fn test_price_helper() {
        let price = Generator::price("10.99", "USD");
        assert_eq!(price.value, "10.99");
        assert_eq!(price.currency_code, "USD");
    }

    #[test]
    fn test_indirect_acquisition_helper() {
        let acquisition = Generator::indirect_acquisition("application/epub+zip");
        assert_eq!(acquisition.r#type, "application/epub+zip");
        assert!(acquisition.child.is_none());
    }

    #[test]
    fn test_nested_indirect_acquisition_helper() {
        let acquisition =
            Generator::nested_indirect_acquisition("application/pdf", "application/epub+zip");

        assert_eq!(acquisition.r#type, "application/pdf");
        assert!(acquisition.child.is_some());
        let child = acquisition.child.unwrap();
        assert_eq!(child.len(), 1);
        assert_eq!(child[0].r#type, "application/epub+zip");
    }

    #[test]
    fn test_feed_with_self_link() {
        let feed =
            Feed::new("urn:uuid:123", "Test Feed").with_self_link("http://example.com/feed.xml");

        assert_eq!(feed.links.len(), 1);
        assert_eq!(feed.links[0].rel, "self");
        assert_eq!(
            feed.links[0].type_,
            Some("application/atom+xml;profile=opds-catalog;kind=navigation".to_string())
        );
    }

    #[test]
    fn test_generate_feed_without_validation() {
        let generator = Generator::new();
        let feed = Generator::generate_minimal_feed(
            "urn:uuid:123",
            "Test Feed",
            "http://example.com/feed.xml",
        );

        // Generate without validation
        let xml = generator.generate_feed(&feed);
        assert!(xml.is_ok());
    }

    #[test]
    fn test_generate_feed_with_validation() {
        let generator = Generator::new();
        let feed = Generator::generate_minimal_feed(
            "urn:uuid:123",
            "Test Feed",
            "http://example.com/feed.xml",
        );

        // Generate with validation
        let xml = generator.generate_and_validate_feed(&feed);
        assert!(xml.is_ok());
    }

    #[test]
    fn test_generate_feed_validation_fails() {
        let generator = Generator::new();

        // Create invalid feed with empty ID
        let mut feed = Feed::new("", "Test Feed");
        let mut entry = Entry::new("urn:uuid:test", "Test Entry");
        entry.links.push(Link::new(
            "http://opds-spec.org/acquisition",
            "http://example.com/book.epub",
        ));
        feed.entries.push(entry);

        // Generate with validation should fail
        let xml = generator.generate_and_validate_feed(&feed);
        assert!(xml.is_err());
        assert!(xml.unwrap_err().to_string().contains("id cannot be empty"));
    }

    #[test]
    fn test_generate_feed_to_writer_without_validation() {
        let generator = Generator::new();
        let feed = Generator::generate_minimal_feed(
            "urn:uuid:123",
            "Test Feed",
            "http://example.com/feed.xml",
        );

        let mut output = Vec::new();
        let result = generator.generate_feed_to_writer(&feed, &mut output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_feed_to_writer_with_validation() {
        let generator = Generator::new();
        let feed = Generator::generate_minimal_feed(
            "urn:uuid:123",
            "Test Feed",
            "http://example.com/feed.xml",
        );

        let mut output = Vec::new();
        let result = generator.generate_and_validate_feed_to_writer(&feed, &mut output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_feed_to_writer_validation_fails() {
        let generator = Generator::new();

        let mut feed = Feed::new("urn:uuid:test", "");
        let mut entry = Entry::new("urn:uuid:test", "Test Entry");
        entry.links.push(Link::new(
            "http://opds-spec.org/acquisition",
            "http://example.com/book.epub",
        ));
        feed.entries.push(entry);

        let mut output = Vec::new();
        let result = generator.generate_and_validate_feed_to_writer(&feed, &mut output);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("title cannot be empty"));
    }

    #[test]
    fn test_validate_feed() {
        let generator = Generator::new();
        let feed = Feed::new("urn:uuid:123", "Test Feed");

        assert!(generator.validate_feed(&feed).is_ok());
    }
}
