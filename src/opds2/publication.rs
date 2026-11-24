//! OPDS 2.0 Publication types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::{Validate, ValidationResult};
use crate::opds2::Link;

/// OPDS 2.0 Publication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Publication {
    /// Publication metadata
    pub metadata: PublicationMetadata,
    /// Publication links (must contain at least one acquisition link)
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    /// Images collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Link>>,
    /// Reading order (for full Web Publications)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reading_order: Option<Vec<Link>>,
    /// Table of contents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toc: Option<Vec<Link>>,
    /// Page list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_list: Option<Vec<Link>>,
}

/// Publication metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicationMetadata {
    /// Identifier for the publication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Type (schema.org type)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Title of the publication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Subtitle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// Last modification date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Items per page (pagination)
    #[serde(
        rename = "itemsPerPage",
        alias = "items_per_page",
        skip_serializing_if = "Option::is_none"
    )]
    pub items_per_page: Option<u32>,
    /// Current page number (pagination)
    #[serde(
        rename = "currentPage",
        alias = "current_page",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_page: Option<u32>,
    /// Total number of items (pagination)
    #[serde(
        rename = "numberOfItems",
        alias = "number_of_items",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_items: Option<u32>,
    /// Author (can be string or object with links)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    /// Translator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translator: Option<String>,
    /// Editor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<String>,
    /// Illustrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illustrator: Option<String>,
    /// Narrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrator: Option<String>,
    /// Publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Published date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,
    /// Series info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<SeriesInfo>,
    /// Collection info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
}

/// Author information (can include links)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    /// Author name
    pub name: String,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<String>,
    /// Links to author resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

/// Series information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesInfo {
    /// Series name
    pub name: String,
    /// Position in series
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
}

impl Default for PublicationMetadata {
    fn default() -> Self {
        Self {
            identifier: None,
            type_: Some("http://schema.org/Book".to_string()),
            title: None,
            subtitle: None,
            modified: None,
            description: None,
            items_per_page: None,
            current_page: None,
            number_of_items: None,
            author: None,
            translator: None,
            editor: None,
            illustrator: None,
            narrator: None,
            publisher: None,
            language: None,
            published: None,
            series: None,
            collection: None,
        }
    }
}

impl Publication {
    /// Create a new publication
    pub fn new(metadata: PublicationMetadata, links: Option<Vec<Link>>) -> Self {
        Self {
            metadata,
            links,
            images: None,
            reading_order: None,
            toc: None,
            page_list: None,
        }
    }

    /// Add images
    pub fn with_images(mut self, images: Vec<Link>) -> Self {
        self.images = Some(images);
        self
    }

    /// Validate the publication
    pub fn validate(&self) -> ValidationResult {
        if self.metadata.title.is_none()
            || self
                .metadata
                .title
                .as_ref()
                .map(|s| s.is_empty())
                .unwrap_or(true)
        {
            return Err(crate::common::error::OpdsError::MissingField(
                "title".to_string(),
            ));
        }

        // Must have at least one acquisition link
        let has_acquisition = self
            .links
            .as_ref()
            .map(|links| {
                links.iter().any(|l| {
                    matches!(
                        l.rel.as_str(),
                        "acquisition" | "borrow" | "buy" | "download" | "preview" | "subscribe"
                    ) || l.rel.starts_with("http://opds-spec.org/acquisition")
                })
            })
            .unwrap_or(false);
        if !has_acquisition {
            return Err(crate::common::error::OpdsError::Validation(
                "Publication must contain at least one acquisition link".to_string(),
            ));
        }

        // Validate all links
        if let Some(ref links) = self.links {
            for link in links {
                link.validate()?;
            }
        }

        // Validate images if present (images is optional)
        if let Some(ref images) = self.images {
            // At least one image must be in a supported format if images collection exists
            let has_valid_image = images.iter().any(|img| {
                img.type_
                    .as_ref()
                    .map(|t| crate::common::is_valid_image_format(t))
                    .unwrap_or(false)
            });
            if !has_valid_image {
                return Err(crate::common::error::OpdsError::Validation(
                    "At least one image must use a supported format (image/jpeg, image/png, image/gif, image/webp, image/avif, image/jxl)"
                        .to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl Validate for Publication {
    fn validate(&self) -> ValidationResult {
        self.validate()
    }
}

impl PublicationMetadata {
    /// Create new publication metadata with title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..Default::default()
        }
    }
}
