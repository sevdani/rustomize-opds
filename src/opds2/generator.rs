//! OPDS 2.0 Generator

use std::io::Write;

use crate::common::error::Result;
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
        serde_json::to_string_pretty(feed).map_err(|e| {
            crate::common::error::OpdsError::Custom(format!(
                "Failed to generate OPDS 2.0 feed: {}",
                e
            ))
        })
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
            crate::common::error::OpdsError::Custom(format!(
                "Failed to generate OPDS 2.0 publication: {}",
                e
            ))
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

    /// Generate a minimal OPDS 2.0 feed
    pub fn generate_minimal_feed(title: impl Into<String>, self_href: impl Into<String>) -> Feed {
        let metadata = crate::common::CommonMetadata::new(title);

        let feed_link =
            crate::opds2::Link::new("self", self_href).with_type("application/opds+json");

        Feed::new(metadata, vec![feed_link])
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

        Publication::new(metadata, vec![self_link, download_link])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_new() {
        let generator = Generator::new();
        assert!(matches!(generator, Generator));
    }

    #[test]
    fn test_generate_minimal_feed() {
        let feed = Generator::generate_minimal_feed("Test Feed", "http://example.com/feed.json");

        assert_eq!(feed.metadata.title, Some("Test Feed".to_string()));
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
            publication.metadata.common.title,
            Some("Test Publication".to_string())
        );
        assert_eq!(publication.links.len(), 2);
        assert_eq!(publication.links[0].rel, "self");
        assert_eq!(publication.links[1].rel, "download");
    }

    #[test]
    fn test_generate_feed_to_json() {
        let generator = Generator::new();
        let feed = Generator::generate_minimal_feed("Test Feed", "http://example.com/feed.json");

        let json = generator.generate_feed(&feed).unwrap();
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

        let json = generator.generate_publication(&publication).unwrap();
        // Check for the title in the JSON output
        assert!(json.contains("Test Publication"));
    }
}
