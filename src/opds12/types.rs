//! OPDS 1.2 Atom types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::{is_valid_currency, OpdsError, ValidationResult};

/// Default datetime helper function for serde default
#[allow(dead_code)]
fn default_datetime() -> DateTime<Utc> {
    Utc::now()
}

/// OPDS 1.2 Feed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "feed")]
pub struct Feed {
    /// Feed identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Feed title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Updated date
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
    /// Feed authors
    #[serde(default, rename = "author", skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Author>>,
    /// Links
    #[serde(default, rename = "link", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// Feed entries
    #[serde(default, rename = "entry", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<Entry>>,
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
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Entry title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Updated date
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
    /// Entry authors
    #[serde(default, rename = "author", skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Author>>,
    /// Links
    #[serde(default, rename = "link", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// Entry summary
    #[serde(default, rename = "summary", skip_serializing_if = "Option::is_none")]
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
    /// Dublin Core identifier elements
    #[serde(
        default,
        rename = "dc:identifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub dc_identifier: Option<Vec<String>>,
    /// Dublin Core title elements
    #[serde(default, rename = "dc:title", skip_serializing_if = "Option::is_none")]
    pub dc_title: Option<Vec<String>>,
    /// Dublin Core language elements
    #[serde(
        default,
        rename = "dc:language",
        skip_serializing_if = "Option::is_none"
    )]
    pub dc_language: Option<Vec<String>>,
    /// Dublin Core issued elements
    #[serde(default, rename = "dc:issued", skip_serializing_if = "Option::is_none")]
    pub dc_issued: Option<Vec<String>>,
    /// Dublin Core publisher elements
    #[serde(
        default,
        rename = "dc:publisher",
        skip_serializing_if = "Option::is_none"
    )]
    pub dc_publisher: Option<Vec<String>>,
    /// Dublin Core subject elements
    #[serde(
        default,
        rename = "dc:subject",
        skip_serializing_if = "Option::is_none"
    )]
    pub dc_subject: Option<Vec<String>>,
    /// Dublin Core description elements
    #[serde(
        default,
        rename = "dc:description",
        skip_serializing_if = "Option::is_none"
    )]
    pub dc_description: Option<Vec<String>>,
    /// Dublin Core creator elements
    #[serde(
        default,
        rename = "dc:creator",
        skip_serializing_if = "Option::is_none"
    )]
    pub dc_creator: Option<Vec<String>>,
    /// Dublin Core rights elements
    #[serde(default, rename = "dc:rights", skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Author {
    /// Create a new author with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uri: None,
            email: None,
            name: Some(name.into()),
        }
    }

    /// Create a new author with URI
    pub fn with_uri(name: impl Into<String>, uri: impl Into<String>) -> Self {
        Self {
            uri: Some(uri.into()),
            email: None,
            name: Some(name.into()),
        }
    }

    /// Create a new author with email
    pub fn with_email(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            uri: None,
            email: Some(email.into()),
            name: Some(name.into()),
        }
    }

    /// Validate the author
    pub fn validate(&self) -> ValidationResult {
        // Validate name is non-empty when present
        if let Some(ref name) = self.name {
            if name.is_empty() {
                return Err(OpdsError::Validation(
                    "author name cannot be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

/// Summary element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "summary")]
#[serde(rename_all = "lowercase")]
pub struct Summary {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl Summary {
    /// Create a new summary with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            type_: Some("text".to_string()),
        }
    }

    /// Create a new summary with type (must be "text" for OPDS 1.2)
    pub fn with_type(text: impl Into<String>, type_: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
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
        // Validate text is non-empty when present
        if let Some(ref text) = self.text {
            if text.is_empty() {
                return Err(OpdsError::Validation(
                    "summary text cannot be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

/// Content element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Content {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl Content {
    /// Create a new content with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            type_: None,
        }
    }

    /// Create a new content with type
    pub fn with_type(text: impl Into<String>, type_: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
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
        // Validate text is non-empty when present
        if let Some(ref text) = self.text {
            if text.is_empty() {
                return Err(OpdsError::Validation(
                    "content text cannot be empty".to_string(),
                ));
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
    #[serde(rename = "@rel", skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    /// Link href
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
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
    #[serde(
        default,
        rename = "indirectAcquisition",
        skip_serializing_if = "Option::is_none"
    )]
    pub indirect_acquisition: Option<Vec<IndirectAcquisition>>,
}

impl Link {
    /// Create a new link
    pub fn new(rel: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            rel: Some(rel.into()),
            href: Some(href.into()),
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
        // rel is required by OPDS 1.2
        let rel = self
            .rel
            .as_ref()
            .ok_or(OpdsError::Validation("link rel is required".to_string()))?;
        if rel.is_empty() {
            return Err(OpdsError::Validation(
                "link rel cannot be empty".to_string(),
            ));
        }

        // href is required by OPDS 1.2
        let href = self
            .href
            .as_ref()
            .ok_or(OpdsError::Validation("link href is required".to_string()))?;
        if href.is_empty() {
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
#[serde(rename = "price")]
#[serde(rename_all = "camelCase")]
pub struct Price {
    #[serde(rename = "@currencycode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Price {
    /// Create a new price
    pub fn new(value: impl Into<String>, currency_code: impl Into<String>) -> Self {
        Self {
            value: Some(value.into()),
            currency_code: Some(currency_code.into()),
        }
    }

    /// Validate the price
    pub fn validate(&self) -> ValidationResult {
        // Validate currency code when present
        if let Some(ref currency) = self.currency_code {
            if !is_valid_currency(currency) {
                return Err(OpdsError::InvalidCurrency(format!(
                    "invalid currency code: {}",
                    currency
                )));
            }
        }

        // Validate value is a non-negative decimal when present
        if let Some(ref val) = self.value {
            match val.parse::<f64>() {
                Ok(num) if num >= 0.0 => Ok(()),
                Ok(_) => Err(OpdsError::Validation(
                    "price value must be non-negative".to_string(),
                )),
                Err(_) => Err(OpdsError::Validation(format!(
                    "price value '{}' is not a valid decimal",
                    val
                ))),
            }
        } else {
            Ok(())
        }
    }
}

/// Indirect acquisition element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "indirectAcquisition")]
pub struct IndirectAcquisition {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(
        default,
        rename = "indirectAcquisition",
        skip_serializing_if = "Option::is_none"
    )]
    pub child: Option<Vec<IndirectAcquisition>>,
}

impl IndirectAcquisition {
    /// Create a new indirect acquisition
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            r#type: Some(r#type.into()),
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
        // Validate type is a non-empty string when present
        if let Some(ref t) = self.r#type {
            if t.is_empty() {
                return Err(OpdsError::Validation(
                    "indirect acquisition type cannot be empty".to_string(),
                ));
            }
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
            id: Some(id.into()),
            title: Some(title.into()),
            updated: Some(Utc::now()),
            authors: Some(Vec::new()),
            links: Some(Vec::new()),
            entries: Some(Vec::new()),
            contributor: None,
            rights: None,
            icon: None,
            logo: None,
            generator: None,
        }
    }

    /// Validate the feed
    pub fn validate(&self) -> ValidationResult {
        // id is required by OPDS 1.2
        let id = self
            .id
            .as_ref()
            .ok_or(OpdsError::MissingField("id".to_string()))?;
        if id.is_empty() {
            return Err(OpdsError::Validation("feed id cannot be empty".to_string()));
        }

        // title is required by OPDS 1.2
        let title = self
            .title
            .as_ref()
            .ok_or(OpdsError::MissingField("title".to_string()))?;
        if title.is_empty() {
            return Err(OpdsError::Validation(
                "feed title cannot be empty".to_string(),
            ));
        }

        // updated is required by OPDS 1.2
        if self.updated.is_none() {
            return Err(OpdsError::MissingField("updated".to_string()));
        }

        // Validate all entries
        if let Some(ref entries) = self.entries {
            for entry in entries {
                entry.validate()?;
            }
        }

        // Validate all links
        if let Some(ref links) = self.links {
            for link in links {
                link.validate()?;
            }
        }

        Ok(())
    }
}

impl Entry {
    /// Create a new entry
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            title: Some(title.into()),
            updated: Some(Utc::now()),
            authors: Some(Vec::new()),
            links: Some(Vec::new()),
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
        // id is required by OPDS 1.2
        let id = self
            .id
            .as_ref()
            .ok_or(OpdsError::MissingField("id".to_string()))?;
        if id.is_empty() {
            return Err(OpdsError::Validation(
                "entry id cannot be empty".to_string(),
            ));
        }

        // title is required by OPDS 1.2
        let title = self
            .title
            .as_ref()
            .ok_or(OpdsError::MissingField("title".to_string()))?;
        if title.is_empty() {
            return Err(OpdsError::Validation(
                "entry title cannot be empty".to_string(),
            ));
        }

        // updated is required by OPDS 1.2
        if self.updated.is_none() {
            return Err(OpdsError::MissingField("updated".to_string()));
        }

        // At least one of content or summary is required by OPDS 1.2
        if self.content.is_none() && self.summary.is_none() {
            return Err(OpdsError::Validation(
                "entry must contain either content or summary".to_string(),
            ));
        }

        // Must have at least one acquisition link
        let has_acquisition = self
            .links
            .as_ref()
            .map(|links| {
                links.iter().any(|l| {
                    l.rel
                        .as_ref()
                        .is_some_and(|r| r.starts_with("http://opds-spec.org/acquisition"))
                })
            })
            .unwrap_or(false);
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
        if let Some(ref links) = self.links {
            for link in links {
                link.validate()?;
            }
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
        assert_eq!(feed.id, Some("urn:uuid:123".to_string()));
        assert_eq!(feed.title, Some("Test Feed".to_string()));
        assert!(feed.authors.as_ref().unwrap().is_empty());
        assert!(feed.links.as_ref().unwrap().is_empty());
        assert!(feed.entries.as_ref().unwrap().is_empty());
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
        assert_eq!(entry.id, Some("urn:uuid:456".to_string()));
        assert_eq!(entry.title, Some("Test Entry".to_string()));
        assert!(entry.authors.as_ref().unwrap().is_empty());
        assert!(entry.links.as_ref().unwrap().is_empty());
    }

    #[test]
    fn test_entry_validation_success() {
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        entry.summary = Some(Summary::new("Summary text"));
        entry.links.as_mut().unwrap().push(Link::new(
            "http://opds-spec.org/acquisition",
            "http://example.com/book.epub",
        ));
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn test_entry_validation_empty_id() {
        let mut entry = Entry::new("", "Test Entry");
        entry
            .links
            .as_mut()
            .unwrap()
            .push(Link::new("self", "http://example.com"));
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_entry_validation_empty_title() {
        let mut entry = Entry::new("urn:uuid:456", "");
        entry
            .links
            .as_mut()
            .unwrap()
            .push(Link::new("self", "http://example.com"));
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_entry_validation_no_acquisition_link() {
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        entry
            .links
            .as_mut()
            .unwrap()
            .push(Link::new("self", "http://example.com"));
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_author_default() {
        let author = Author {
            name: Some("Test Author".to_string()),
            uri: None,
            email: None,
        };
        assert_eq!(author.name, Some("Test Author".to_string()));
    }

    #[test]
    fn test_summary_default() {
        let summary = Summary {
            text: Some("Summary text".to_string()),
            type_: None,
        };
        assert_eq!(summary.text, Some("Summary text".to_string()));
    }

    #[test]
    fn test_content_default() {
        let content = Content {
            text: Some("Content text".to_string()),
            type_: None,
        };
        assert_eq!(content.text, Some("Content text".to_string()));
    }

    #[test]
    fn test_link_new() {
        let link = Link::new("self", "http://example.com");
        assert_eq!(link.rel, Some("self".to_string()));
        assert_eq!(link.href, Some("http://example.com".to_string()));
    }

    #[test]
    fn test_price_default() {
        let price = Price {
            value: Some("10.99".to_string()),
            currency_code: Some("USD".to_string()),
        };
        assert_eq!(price.value, Some("10.99".to_string()));
        assert_eq!(price.currency_code, Some("USD".to_string()));
    }

    #[test]
    fn test_indirect_acquisition_default() {
        let acquisition = IndirectAcquisition {
            r#type: Some("application/epub+zip".to_string()),
            child: None,
        };
        assert_eq!(acquisition.r#type, Some("application/epub+zip".to_string()));
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
        entry.summary = Some(Summary::new("Summary text"));
        entry.links.as_mut().unwrap().push(
            Link::new("http://opds-spec.org/acquisition/buy", "http://example.com")
                .with_price(Price::new("10.99", "USD")),
        );
        let result = entry.validate();
        assert!(result.is_ok(), "Validation failed: {:?}", result);
    }

    #[test]
    fn test_entry_validation_with_invalid_price() {
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        entry.links.as_mut().unwrap().push(
            Link::new("http://opds-spec.org/acquisition/buy", "http://example.com")
                .with_price(Price::new("-5.00", "USD")),
        );
        assert!(entry.validate().is_err());
    }

    #[test]
    fn test_feed_validation_with_entries() {
        let mut feed = Feed::new("urn:uuid:123", "Test Feed");
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        entry.summary = Some(Summary::new("Summary text"));
        entry.links.as_mut().unwrap().push(Link::new(
            "http://opds-spec.org/acquisition",
            "http://example.com/book.epub",
        ));
        feed.entries.as_mut().unwrap().push(entry);
        assert!(feed.validate().is_ok());
    }

    #[test]
    fn test_feed_validation_with_invalid_entry() {
        let mut feed = Feed::new("urn:uuid:123", "Test Feed");
        let mut entry = Entry::new("urn:uuid:456", "Test Entry");
        // No acquisition link - should fail validation
        entry
            .links
            .as_mut()
            .unwrap()
            .push(Link::new("self", "http://example.com"));
        feed.entries.as_mut().unwrap().push(entry);
        assert!(feed.validate().is_err());
    }
}
