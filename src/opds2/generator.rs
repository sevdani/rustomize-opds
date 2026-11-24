//! OPDS 2.0 Generator

use std::io::Write;

use crate::common::error::{OpdsError, Result, ValidationResult};
use crate::opds2::{Feed, Publication};

/// OPDS 2.0 Generator
pub struct Generator;

impl Generator {
    /// Create a new OPDS 2.0 generator
    pub fn new() -> Self {
        Self
    }

    /// Generate OPDS 2.0 feed to JSON string
    pub fn generate_feed(&self, feed: &Feed) -> Result<String> {
        serde_json::to_string_pretty(feed)
            .map_err(|e| OpdsError::Custom(format!("Failed to generate OPDS 2.0 feed: {}", e)))
    }

    /// Generate OPDS 2.0 feed to writer
    pub fn generate_feed_to_writer<W: Write>(&self, feed: &Feed, mut writer: W) -> Result<()> {
        let json = serde_json::to_string_pretty(feed)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Generate OPDS 2.0 publication to JSON string
    pub fn generate_publication(&self, pub_: &Publication) -> Result<String> {
        serde_json::to_string_pretty(pub_).map_err(|e| {
            OpdsError::Custom(format!("Failed to generate OPDS 2.0 publication: {}", e))
        })
    }

    /// Generate OPDS 2.0 publication to writer
    pub fn generate_publication_to_writer<W: Write>(
        &self,
        pub_: &Publication,
        mut writer: W,
    ) -> Result<()> {
        let json = serde_json::to_string_pretty(pub_)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Generate OPDS 2.0 feed to JSON string with auto-validation
    pub fn generate_and_validate_feed(&self, feed: &Feed) -> Result<String> {
        feed.validate()?;
        serde_json::to_string_pretty(feed)
            .map_err(|e| OpdsError::Custom(format!("Failed to generate OPDS 2.0 feed: {}", e)))
    }

    /// Generate OPDS 2.0 feed to writer with auto-validation
    pub fn generate_and_validate_feed_to_writer<W: Write>(
        &self,
        feed: &Feed,
        mut writer: W,
    ) -> Result<()> {
        feed.validate()?;
        let json = serde_json::to_string_pretty(feed)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Generate OPDS 2.0 publication to JSON string with auto-validation
    pub fn generate_and_validate_publication(&self, pub_: &Publication) -> Result<String> {
        pub_.validate()?;
        serde_json::to_string_pretty(pub_).map_err(|e| {
            OpdsError::Custom(format!("Failed to generate OPDS 2.0 publication: {}", e))
        })
    }

    /// Generate OPDS 2.0 publication to writer with auto-validation
    pub fn generate_and_validate_publication_to_writer<W: Write>(
        &self,
        pub_: &Publication,
        mut writer: W,
    ) -> Result<()> {
        pub_.validate()?;
        let json = serde_json::to_string_pretty(pub_)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Validate a parsed feed
    pub fn validate_feed(&self, feed: &Feed) -> ValidationResult {
        feed.validate()
    }

    /// Validate a parsed publication
    pub fn validate_publication(&self, pub_: &Publication) -> ValidationResult {
        pub_.validate()
    }

    /// Generate a minimal OPDS 2.0 feed
    pub fn generate_minimal_feed(title: impl Into<String>, self_href: impl Into<String>) -> Feed {
        let feed_link =
            crate::opds2::Link::new("self", self_href).with_type("application/opds+json");

        Feed::new(title, vec![feed_link])
    }

    /// Generate a minimal OPDS 2.0 publication
    pub fn generate_minimal_publication(
        title: impl Into<String>,
        self_href: impl Into<String>,
        download_href: impl Into<String>,
    ) -> Publication {
        let metadata = crate::opds2::PublicationMetadata::new(title);

        let self_link = crate::opds2::Link::new("self", self_href)
            .with_type("application/opds-publication+json");
        let download_link = crate::opds2::Link::new("download", download_href);

        Publication::new(metadata, Some(vec![self_link, download_link]))
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for Link to set media type
pub trait LinkWithType {
    /// Set the media type on a link
    fn with_type(self, type_: impl Into<String>) -> Self;
}

impl LinkWithType for crate::opds2::Link {
    fn with_type(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }
}

/// Extension trait for Link to set additional properties
pub trait LinkWithProperties {
    /// Set the title on a link
    fn with_title(self, title: impl Into<String>) -> Self;
    /// Set the height on a link
    fn with_height(self, height: u32) -> Self;
    /// Set the width on a link
    fn with_width(self, width: u32) -> Self;
    /// Set the properties on a link
    fn with_properties(self, properties: crate::opds2::LinkProperties) -> Self;
}

impl LinkWithProperties for crate::opds2::Link {
    fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    fn with_properties(mut self, properties: crate::opds2::LinkProperties) -> Self {
        self.properties = Some(properties);
        self
    }
}

/// Extension trait for Feed to add publications
pub trait FeedWithPublications {
    /// Add publications to a feed
    fn with_publications(self, publications: Vec<crate::opds2::Publication>) -> Self;
}

impl FeedWithPublications for crate::opds2::Feed {
    fn with_publications(mut self, publications: Vec<crate::opds2::Publication>) -> Self {
        self.publications = Some(publications);
        self
    }
}

/// Extension trait for Publication to add links
pub trait PublicationWithLink {
    /// Add a link to a publication
    fn with_link(self, link: crate::opds2::Link) -> Self;
}

impl PublicationWithLink for crate::opds2::Publication {
    fn with_link(mut self, link: crate::opds2::Link) -> Self {
        if self.links.is_none() {
            self.links = Some(vec![]);
        }
        self.links.as_mut().unwrap().push(link);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opds2::{Feed, Link, Publication, PublicationMetadata};

    #[test]
    fn test_generator_new() {
        let generator = Generator::new();
        assert!(matches!(generator, Generator));
    }

    #[test]
    fn test_generate_minimal_feed() {
        let feed = Generator::generate_minimal_feed("Test Feed", "http://example.com/feed.json");

        assert_eq!(feed.title, Some("Test Feed".to_string()));
        assert_eq!(feed.feed_links.len(), 1);
        assert_eq!(feed.feed_links[0].rel, "self");
        assert_eq!(feed.feed_links[0].href, "http://example.com/feed.json");
    }

    #[test]
    fn test_generate_minimal_publication() {
        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );

        assert_eq!(
            publication.metadata.title,
            Some("Test Publication".to_string())
        );
        let links = publication.links.as_ref().unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].rel, "self");
        assert_eq!(links[1].rel, "download");
    }

    #[test]
    fn test_generate_feed_to_json() {
        let generator = Generator::new();
        let feed = Generator::generate_minimal_feed("Test Feed", "http://example.com/feed.json");

        // Add publications to make it a valid OPDS 2.0 feed
        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );
        let feed = feed.with_publications(vec![publication]);

        let json = generator.generate_and_validate_feed(&feed).unwrap();
        // Check for the title in the JSON output
        assert!(json.contains("Test Feed"));
        assert!(json.contains("\"self\""));
    }

    #[test]
    fn test_generate_publication_to_json() {
        let generator = Generator::new();
        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );

        let json = generator
            .generate_and_validate_publication(&publication)
            .unwrap();
        // Check for the title in the JSON output
        assert!(json.contains("Test Publication"));
    }

    #[test]
    fn test_generate_feed_without_validation() {
        let generator = Generator::new();

        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );
        let feed = Generator::generate_minimal_feed("Test Feed", "http://example.com/feed.json")
            .with_publications(vec![publication]);

        let json = generator.generate_feed(&feed);
        assert!(json.is_ok());
    }

    #[test]
    fn test_generate_feed_with_validation() {
        let generator = Generator::new();

        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );
        let feed = Generator::generate_minimal_feed("Test Feed", "http://example.com/feed.json")
            .with_publications(vec![publication]);

        let json = generator.generate_and_validate_feed(&feed);
        assert!(json.is_ok());
    }

    #[test]
    fn test_generate_feed_validation_fails() {
        let generator = Generator::new();

        // Create invalid feed without publications (required for OPDS 2.0)
        let feed = Feed::new(
            "Test Feed",
            vec![Link::new("self", "http://example.com/feed.json")],
        );

        let json = generator.generate_and_validate_feed(&feed);
        assert!(json.is_err());
        assert!(json
            .unwrap_err()
            .to_string()
            .contains("navigation, publications, or groups"));
    }

    #[test]
    fn test_generate_publication_without_validation() {
        let generator = Generator::new();
        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );

        let json = generator.generate_publication(&publication);
        assert!(json.is_ok());
    }

    #[test]
    fn test_generate_publication_with_validation() {
        let generator = Generator::new();
        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );

        let json = generator.generate_and_validate_publication(&publication);
        assert!(json.is_ok());
    }

    #[test]
    fn test_generate_publication_validation_fails() {
        let generator = Generator::new();

        // Create invalid publication without acquisition link
        let metadata = PublicationMetadata::new("Test Publication");
        let publication = Publication::new(
            metadata,
            Some(vec![Link::new("self", "http://example.com")]),
        );

        let json = generator.generate_and_validate_publication(&publication);
        assert!(json.is_err());
        assert!(json.unwrap_err().to_string().contains("acquisition link"));
    }

    #[test]
    fn test_generate_feed_to_writer_without_validation() {
        let generator = Generator::new();

        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );
        let feed = Generator::generate_minimal_feed("Test Feed", "http://example.com/feed.json")
            .with_publications(vec![publication]);

        let mut output = Vec::new();
        let result = generator.generate_feed_to_writer(&feed, &mut output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_feed_to_writer_with_validation() {
        let generator = Generator::new();

        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );
        let feed = Generator::generate_minimal_feed("Test Feed", "http://example.com/feed.json")
            .with_publications(vec![publication]);

        let mut output = Vec::new();
        let result = generator.generate_and_validate_feed_to_writer(&feed, &mut output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_publication_to_writer_without_validation() {
        let generator = Generator::new();
        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );

        let mut output = Vec::new();
        let result = generator.generate_publication_to_writer(&publication, &mut output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_publication_to_writer_with_validation() {
        let generator = Generator::new();
        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );

        let mut output = Vec::new();
        let result =
            generator.generate_and_validate_publication_to_writer(&publication, &mut output);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_feed() {
        let generator = Generator::new();

        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );
        let feed = Generator::generate_minimal_feed("Test Feed", "http://example.com/feed.json")
            .with_publications(vec![publication]);

        assert!(generator.validate_feed(&feed).is_ok());
    }

    #[test]
    fn test_validate_publication() {
        let generator = Generator::new();
        let publication = Generator::generate_minimal_publication(
            "Test Publication",
            "http://example.com/pub.json",
            "http://example.com/book.epub",
        );

        assert!(generator.validate_publication(&publication).is_ok());
    }
}
