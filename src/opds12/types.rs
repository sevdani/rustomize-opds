//! OPDS 1.2 Atom types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::{is_valid_currency, OpdsError, ValidationResult};

/// OPDS 1.2 Feed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "feed")]
pub struct Feed {
    /// Feed identifier
    #[serde(rename = "id")]
    pub id: String,
    /// Feed title
    pub title: String,
    /// Updated date
    #[serde(rename = "updated")]
    pub updated: DateTime<Utc>,
    /// Feed authors
    #[serde(default, rename = "author")]
    pub authors: Vec<Author>,
    /// Links
    #[serde(default, rename = "link")]
    pub links: Vec<Link>,
    /// Feed entries
    #[serde(default, rename = "entry")]
    pub entries: Vec<Entry>,
    /// Feed contributor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributor: Option<String>,
    /// Rights
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,
    /// Icon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Logo
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Generator
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
}

/// OPDS 1.2 Entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "entry")]
pub struct Entry {
    /// Entry identifier
    #[serde(rename = "id")]
    pub id: String,
    /// Entry title
    pub title: String,
    /// Updated date
    #[serde(rename = "updated")]
    pub updated: DateTime<Utc>,
    /// Entry authors
    #[serde(default, rename = "author")]
    pub authors: Vec<Author>,
    /// Links
    #[serde(default, rename = "link")]
    pub links: Vec<Link>,
    /// Entry summary
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<Summary>,
    /// Entry content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    /// Published date
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<DateTime<Utc>>,
    /// Entry contributor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributor: Option<String>,
    /// Rights
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,
    /// Dublin Core metadata (via dc: prefix)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_identifier: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_title: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_language: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_issued: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_publisher: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_subject: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_description: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_creator: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_rights: Option<Vec<String>>,
}

/// Author element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "author")]
pub struct Author {
    /// Author URI
    #[serde(rename = "@uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Author email
    #[serde(rename = "@email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Author name
    pub name: String,
}

/// Summary element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Summary {
    #[serde(rename = "$text")]
    pub text: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl Summary {
    /// Create a new summary with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            type_: None,
        }
    }

    /// Create a new summary with type (must be "text" for OPDS 1.2)
    pub fn with_type(text: impl Into<String>, type_: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            type_: Some(type_.into()),
        }
    }

    /// Validate the summary
    pub fn validate(&self) -> ValidationResult {
        // For OPDS 1.2, summary type must be "text" (no HTML)
        if let Some(ref type_str) = self.type_ {
            if type_str != "text" {
                return Err(OpdsError::Validation(
                    "summary type must be 'text' (no HTML allowed in OPDS 1.2)".to_string(),
                ));
            }
        }
        Ok(())
    }
}

/// Content element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Content {
    #[serde(rename = "$text")]
    pub text: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl Content {
    /// Create a new content with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            type_: None,
        }
    }

    /// Create a new content with type
    pub fn with_type(text: impl Into<String>, type_: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            type_: Some(type_.into()),
        }
    }

    /// Validate the content
    pub fn validate(&self) -> ValidationResult {
        // For OPDS 1.2, content type (if specified) must be "text" or "html"
        // but summary should be text-only
        if let Some(ref type_str) = self.type_ {
            if type_str != "text" && type_str != "html" {
                return Err(OpdsError::Validation(format!(
                    "content type '{}' is not valid (must be 'text' or 'html')",
                    type_str
                )));
            }
        }
        Ok(())
    }
}

/// Link element (Atom + OPDS extensions)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "link")]
pub struct Link {
    /// Link relation
    #[serde(rename = "@rel")]
    pub rel: String,
    /// Link href
    #[serde(rename = "@href")]
    pub href: String,
    /// Link type (media type)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Link hreflang
    #[serde(rename = "@hreflang", skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<String>,
    /// Link title
    #[serde(rename = "@title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Link length
    #[serde(rename = "@length", skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    /// OPDS facet group attribute
    #[serde(rename = "@opds_facet_group", skip_serializing_if = "Option::is_none")]
    pub opds_facet_group: Option<String>,
    /// OPDS active facet attribute
    #[serde(rename = "@opds_active_facet", skip_serializing_if = "Option::is_none")]
    pub opds_active_facet: Option<String>,
    /// OPDS price element
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Vec<Price>>,
    /// OPDS indirect acquisition element
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indirect_acquisition: Option<Vec<IndirectAcquisition>>,
}

impl Link {
    /// Create a new link
    pub fn new(rel: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            rel: rel.into(),
            href: href.into(),
            type_: None,
            hreflang: None,
            title: None,
            length: None,
            opds_facet_group: None,
            opds_active_facet: None,
            price: None,
            indirect_acquisition: None,
        }
    }

    /// Validate the link
    pub fn validate(&self) -> ValidationResult {
        // Validate href is a non-empty string
        if self.href.is_empty() {
            return Err(OpdsError::Validation(
                "link href cannot be empty".to_string(),
            ));
        }

        // Validate price elements if present
        if let Some(ref prices) = self.price {
            for price in prices {
                price.validate()?;
            }
        }

        // Validate indirect acquisitions if present
        if let Some(ref acquisitions) = self.indirect_acquisition {
            for acquisition in acquisitions {
                acquisition.validate()?;
            }
        }

        Ok(())
    }
}

/// Price element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Price {
    #[serde(rename = "currencycode")]
    pub currency_code: String,
    #[serde(rename = "$text")]
    pub value: String,
}

impl Price {
    /// Create a new price
    pub fn new(value: impl Into<String>, currency_code: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            currency_code: currency_code.into(),
        }
    }

    /// Validate the price
    pub fn validate(&self) -> ValidationResult {
        // Validate currency code
        if !is_valid_currency(&self.currency_code) {
            return Err(OpdsError::InvalidCurrency(format!(
                "invalid currency code: {}",
                self.currency_code
            )));
        }

        // Validate value is a non-negative decimal
        match self.value.parse::<f64>() {
            Ok(num) if num >= 0.0 => Ok(()),
            Ok(_) => Err(OpdsError::Validation(
                "price value must be non-negative".to_string(),
            )),
            Err(_) => Err(OpdsError::Validation(format!(
                "price value '{}' is not a valid decimal",
                self.value
            ))),
        }
    }
}

/// Indirect acquisition element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndirectAcquisition {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub child: Option<Vec<IndirectAcquisition>>,
}

impl IndirectAcquisition {
    /// Create a new indirect acquisition
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            child: None,
        }
    }

    /// Add a nested indirect acquisition
    pub fn with_child(mut self, child: IndirectAcquisition) -> Self {
        if self.child.is_none() {
            self.child = Some(vec![]);
        }
        self.child.as_mut().unwrap().push(child);
        self
    }

    /// Validate the indirect acquisition
    pub fn validate(&self) -> ValidationResult {
        // Validate type is a non-empty string
        if self.r#type.is_empty() {
            return Err(OpdsError::Validation(
                "indirect acquisition type cannot be empty".to_string(),
            ));
        }

        // Validate nested children recursively
        if let Some(ref children) = self.child {
            for child in children {
                child.validate()?;
            }
        }

        Ok(())
    }
}

impl Feed {
    /// Create a new feed
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            updated: Utc::now(),
            authors: Vec::new(),
            links: Vec::new(),
            entries: Vec::new(),
            contributor: None,
            rights: None,
            icon: None,
            logo: None,
            generator: None,
        }
    }

    /// Validate the feed
    pub fn validate(&self) -> ValidationResult {
        if self.id.is_empty() {
            return Err(OpdsError::Validation("feed id cannot be empty".to_string()));
        }
        if self.title.is_empty() {
            return Err(OpdsError::Validation(
                "feed title cannot be empty".to_string(),
            ));
        }

        // Validate all entries
        for entry in &self.entries {
            entry.validate()?;
        }

        // Validate all links
        for link in &self.links {
            link.validate()?;
        }

        Ok(())
    }
}

impl Entry {
    /// Create a new entry
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            updated: Utc::now(),
            authors: Vec::new(),
            links: Vec::new(),
            summary: None,
            content: None,
            published: None,
            contributor: None,
            rights: None,
            dc_identifier: None,
            dc_title: None,
            dc_language: None,
            dc_issued: None,
            dc_publisher: None,
            dc_subject: None,
            dc_description: None,
            dc_creator: None,
            dc_rights: None,
        }
    }

    /// Validate the entry
    pub fn validate(&self) -> ValidationResult {
        if self.id.is_empty() {
            return Err(OpdsError::Validation(
                "entry id cannot be empty".to_string(),
            ));
        }
        if self.title.is_empty() {
            return Err(OpdsError::Validation(
                "entry title cannot be empty".to_string(),
            ));
        }
        // Must have at least one acquisition link
        let has_acquisition = self
            .links
            .iter()
            .any(|l| l.rel.starts_with("http://opds-spec.org/acquisition"));
        if !has_acquisition {
            return Err(OpdsError::Validation(
                "entry must contain at least one acquisition link".to_string(),
            ));
        }

        // Validate summary if present
        if let Some(ref summary) = self.summary {
            summary.validate()?;
        }

        // Validate content if present
        if let Some(ref content) = self.content {
            content.validate()?;
        }

        // Validate all links
        for link in &self.links {
            link.validate()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opds12::generator::LinkWithProperties;

    #[test]
    fn test_feed_new() {
        let feed = Feed::new("urn:uuid:123", "Test Feed");
        assert_eq!(feed.id, "urn:uuid:123");
        assert_eq!(feed.title, "Test Feed");
        assert!(feed.authors.is_empty());
        assert!(feed.links.is_empty());
        assert!(feed.entries.is_empty());
    }

    #[test]
    fn test_feed_validation_success() {
        let feed = Feed::new("urn:uuid:123", "Test Feed");
        assert!(feed.validate().is_ok());
    }

    #[test]
    fn test_feed_validation_empty_id() {
        let feed = Feed::new("", "Test Feed");
        assert!(feed.validate().is_err());
    }

    #[test]
    fn test_feed_validation_empty_title() {
        let feed = Feed::new("urn:uuid:123", "");
        assert!(feed.validate().is_err());
    }

    #[test]
    fn test_entry_new() {
        let entry = Entry::new("urn:uuid:456", "Test Entry");
        assert_eq!(entry.id, "urn:uuid:456");
        assert_eq!(entry.title, "Test Entry");
        assert!(entry.authors.is_empty());
        assert!(entry.links.is_empty());
    }

    #[test]
    fn test_entry_validation_success() {
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        entry.links.push(Link::new(
            "http://opds-spec.org/acquisition",
            "http://example.com/book.epub",
        ));
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn test_entry_validation_empty_id() {
        let mut entry = Entry::new("", "Test Entry");
        entry.links.push(Link::new("self", "http://example.com"));
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_entry_validation_empty_title() {
        let mut entry = Entry::new("urn:uuid:456", "");
        entry.links.push(Link::new("self", "http://example.com"));
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_entry_validation_no_acquisition_link() {
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        entry.links.push(Link::new("self", "http://example.com"));
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_author_default() {
        let author = Author {
            name: "Test Author".to_string(),
            uri: None,
            email: None,
        };
        assert_eq!(author.name, "Test Author");
    }

    #[test]
    fn test_summary_default() {
        let summary = Summary {
            text: "Summary text".to_string(),
            type_: None,
        };
        assert_eq!(summary.text, "Summary text");
    }

    #[test]
    fn test_content_default() {
        let content = Content {
            text: "Content text".to_string(),
            type_: None,
        };
        assert_eq!(content.text, "Content text");
    }

    #[test]
    fn test_link_new() {
        let link = Link::new("self", "http://example.com");
        assert_eq!(link.rel, "self");
        assert_eq!(link.href, "http://example.com");
    }

    #[test]
    fn test_price_default() {
        let price = Price {
            value: "10.99".to_string(),
            currency_code: "USD".to_string(),
        };
        assert_eq!(price.value, "10.99");
        assert_eq!(price.currency_code, "USD");
    }

    #[test]
    fn test_indirect_acquisition_default() {
        let acquisition = IndirectAcquisition {
            r#type: "application/epub+zip".to_string(),
            child: None,
        };
        assert_eq!(acquisition.r#type, "application/epub+zip");
    }

    #[test]
    fn test_price_validation_success() {
        let price = Price::new("10.99", "USD");
        assert!(price.validate().is_ok());
    }

    #[test]
    fn test_price_validation_negative() {
        let price = Price::new("-5.00", "USD");
        assert!(price.validate().is_err());
    }

    #[test]
    fn test_price_validation_invalid_currency() {
        let price = Price::new("10.99", "INVALID");
        assert!(price.validate().is_err());
    }

    #[test]
    fn test_price_validation_non_decimal() {
        let price = Price::new("abc", "USD");
        assert!(price.validate().is_err());
    }

    #[test]
    fn test_indirect_acquisition_validation_success() {
        let acquisition = IndirectAcquisition::new("application/epub+zip");
        assert!(acquisition.validate().is_ok());
    }

    #[test]
    fn test_indirect_acquisition_validation_empty_type() {
        let acquisition = IndirectAcquisition::new("");
        assert!(acquisition.validate().is_err());
    }

    #[test]
    fn test_indirect_acquisition_validation_nested() {
        let acquisition = IndirectAcquisition::new("application/vnd.adobe.adept+xml")
            .with_child(IndirectAcquisition::new("application/pdf"));
        assert!(acquisition.validate().is_ok());
    }

    #[test]
    fn test_indirect_acquisition_validation_nested_invalid() {
        let acquisition = IndirectAcquisition::new("application/vnd.adobe.adept+xml")
            .with_child(IndirectAcquisition::new(""));
        assert!(acquisition.validate().is_err());
    }

    #[test]
    fn test_summary_validation_text_only() {
        let summary = Summary::with_type("Summary text", "text");
        assert!(summary.validate().is_ok());
    }

    #[test]
    fn test_summary_validation_invalid_type() {
        let summary = Summary::with_type("Summary text", "html");
        assert!(summary.validate().is_err());
    }

    #[test]
    fn test_summary_validation_none_type() {
        let summary = Summary::new("Summary text");
        assert!(summary.validate().is_ok());
    }

    #[test]
    fn test_content_validation_text() {
        let content = Content::with_type("Content text", "text");
        assert!(content.validate().is_ok());
    }

    #[test]
    fn test_content_validation_html() {
        let content = Content::with_type("<p>HTML content</p>", "html");
        assert!(content.validate().is_ok());
    }

    #[test]
    fn test_content_validation_invalid_type() {
        let content = Content::with_type("Content text", "invalid");
        assert!(content.validate().is_err());
    }

    #[test]
    fn test_link_validation_success() {
        let link = Link::new("self", "http://example.com");
        assert!(link.validate().is_ok());
    }

    #[test]
    fn test_link_validation_empty_href() {
        let link = Link::new("self", "");
        assert!(link.validate().is_err());
    }

    #[test]
    fn test_link_validation_with_price() {
        let link = Link::new("buy", "http://example.com").with_price(Price::new("10.99", "USD"));
        assert!(link.validate().is_ok());
    }

    #[test]
    fn test_link_validation_with_invalid_price() {
        let link = Link::new("buy", "http://example.com").with_price(Price::new("-5.00", "USD"));
        assert!(link.validate().is_err());
    }

    #[test]
    fn test_link_validation_with_indirect_acquisition() {
        let link = Link::new("acquisition", "http://example.com")
            .with_indirect_acquisition(IndirectAcquisition::new("application/epub+zip"));
        assert!(link.validate().is_ok());
    }

    #[test]
    fn test_entry_validation_with_price() {
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        entry.links.push(
            Link::new("http://opds-spec.org/acquisition/buy", "http://example.com")
                .with_price(Price::new("10.99", "USD")),
        );
        let result = entry.validate();
        assert!(result.is_ok(), "Validation failed: {:?}", result);
    }

    #[test]
    fn test_entry_validation_with_invalid_price() {
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        entry.links.push(
            Link::new("http://opds-spec.org/acquisition/buy", "http://example.com")
                .with_price(Price::new("-5.00", "USD")),
        );
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_feed_validation_with_entries() {
        let mut feed = Feed::new("urn:uuid:123", "Test Feed");
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        entry.links.push(Link::new(
            "http://opds-spec.org/acquisition",
            "http://example.com/book.epub",
        ));
        feed.entries.push(entry);
        assert!(feed.validate().is_ok());
    }

    #[test]
    fn test_feed_validation_with_invalid_entry() {
        let mut feed = Feed::new("urn:uuid:123", "Test Feed");
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        // No acquisition link - should fail validation
        entry.links.push(Link::new("self", "http://example.com"));
        feed.entries.push(entry);
        assert!(feed.validate().is_err());
    }
}
