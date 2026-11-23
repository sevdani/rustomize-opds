//! OPDS 2.0 Feed types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::{Validate, ValidationResult};
use crate::opds2::{Link, Publication};

/// OPDS 2.0 Feed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Feed {
    /// Feed identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Type (schema.org type)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Feed title
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
    /// Feed links
    #[serde(rename = "links")]
    pub feed_links: Vec<Link>,
    /// Navigation collection (compact)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation: Option<Vec<Link>>,
    /// Publications collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>,
    /// Images collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Link>>,
    /// Facets collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<Facet>>,
    /// Groups collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    /// Additional properties (for extensibility)
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

/// Facet group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Facet {
    /// Facet identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Type (schema.org type)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Facet title
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
    /// Facet links (required)
    #[serde(rename = "links")]
    pub links: Vec<Link>,
    /// Images collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Link>>,
}

/// Group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Group {
    /// Group identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Type (schema.org type)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Group title
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
    /// Navigation collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation: Option<Vec<Link>>,
    /// Publications collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>,
    /// Images collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Link>>,
    /// Links collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl Feed {
    /// Create a new feed with required fields
    pub fn new(title: impl Into<String>, feed_links: Vec<Link>) -> Self {
        Self {
            identifier: None,
            type_: None,
            title: Some(title.into()),
            subtitle: None,
            modified: None,
            description: None,
            items_per_page: None,
            current_page: None,
            number_of_items: None,
            feed_links,
            navigation: None,
            publications: None,
            images: None,
            facets: None,
            groups: None,
            additional_properties: HashMap::new(),
        }
    }

    /// Add navigation links
    pub fn with_navigation(mut self, navigation: Vec<Link>) -> Self {
        self.navigation = Some(navigation);
        self
    }

    /// Add publications
    pub fn with_publications(mut self, publications: Vec<Publication>) -> Self {
        self.publications = Some(publications);
        self
    }

    /// Validate the feed
    pub fn validate(&self) -> ValidationResult {
        if self.title.is_none() || self.title.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            return Err(crate::common::error::OpdsError::MissingField(
                "title".to_string(),
            ));
        }

        // Must have at least one collection
        if self.navigation.is_none() && self.publications.is_none() && self.groups.is_none() {
            return Err(crate::common::error::OpdsError::Validation(
                "Feed must contain at least one of: navigation, publications, or groups"
                    .to_string(),
            ));
        }

        // Must have self link
        let has_self_link = self.feed_links.iter().any(|l| l.rel == "self");
        if !has_self_link {
            return Err(crate::common::error::OpdsError::Validation(
                "Feed must contain a self link".to_string(),
            ));
        }

        // Validate all links
        for link in &self.feed_links {
            link.validate()?;
        }

        // Validate publications if present
        if let Some(ref pubs) = self.publications {
            for pub_ in pubs {
                pub_.validate()?;
            }
        }

        // Validate facets if present
        if let Some(ref facets) = self.facets {
            for facet in facets {
                facet.validate()?;
            }
        }

        // Validate groups if present
        if let Some(ref groups) = self.groups {
            for group in groups {
                group.validate()?;
            }
        }

        // Validate navigation if present - each navigation link must have a title
        if let Some(ref navigation) = self.navigation {
            for link in navigation {
                if link.title.is_none() {
                    return Err(crate::common::error::OpdsError::Validation(
                        "Navigation link must contain a title".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}

impl Validate for Feed {
    fn validate(&self) -> ValidationResult {
        self.validate()
    }
}

impl Facet {
    /// Create a new facet
    pub fn new(title: impl Into<String>, links: Vec<Link>) -> Self {
        Self {
            identifier: None,
            type_: None,
            title: Some(title.into()),
            subtitle: None,
            modified: None,
            description: None,
            items_per_page: None,
            current_page: None,
            number_of_items: None,
            links,
            images: None,
        }
    }

    /// Validate the facet
    pub fn validate(&self) -> ValidationResult {
        if self.title.is_none() || self.title.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            return Err(crate::common::error::OpdsError::MissingField(
                "title".to_string(),
            ));
        }

        // Links must have at least one item (minItems: 1)
        if self.links.is_empty() {
            return Err(crate::common::error::OpdsError::Validation(
                "Facet must contain at least one link".to_string(),
            ));
        }

        Ok(())
    }
}

impl Validate for Facet {
    fn validate(&self) -> ValidationResult {
        self.validate()
    }
}

impl Group {
    /// Create a new group
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            identifier: None,
            type_: None,
            title: Some(title.into()),
            subtitle: None,
            modified: None,
            description: None,
            items_per_page: None,
            current_page: None,
            number_of_items: None,
            navigation: None,
            publications: None,
            images: None,
            links: None,
        }
    }

    /// Validate the group
    pub fn validate(&self) -> ValidationResult {
        if self.title.is_none() || self.title.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            return Err(crate::common::error::OpdsError::MissingField(
                "title".to_string(),
            ));
        }

        // Must have at least one collection
        if self.navigation.is_none() && self.publications.is_none() && self.links.is_none() {
            return Err(crate::common::error::OpdsError::Validation(
                "Group must contain at least one of: navigation, publications, or links"
                    .to_string(),
            ));
        }

        Ok(())
    }
}

impl Validate for Group {
    fn validate(&self) -> ValidationResult {
        self.validate()
    }
}
