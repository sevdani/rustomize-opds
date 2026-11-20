//! OPDS 2.0 Feed types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::{CommonMetadata, Validate, ValidationResult};
use crate::opds2::{Link, Publication};

/// OPDS 2.0 Feed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Feed {
    /// Feed metadata
    pub metadata: CommonMetadata,
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
    /// Facet metadata
    pub metadata: CommonMetadata,
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
    /// Group metadata
    pub metadata: CommonMetadata,
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
    pub fn new(metadata: CommonMetadata, feed_links: Vec<Link>) -> Self {
        Self {
            metadata,
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
        self.metadata.validate()?;

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
    pub fn new(metadata: CommonMetadata, links: Vec<Link>) -> Self {
        Self {
            metadata,
            links,
            images: None,
        }
    }

    /// Validate the facet
    pub fn validate(&self) -> ValidationResult {
        self.metadata.validate()?;

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
    pub fn new(metadata: CommonMetadata) -> Self {
        Self {
            metadata,
            navigation: None,
            publications: None,
            images: None,
            links: None,
        }
    }

    /// Validate the group
    pub fn validate(&self) -> ValidationResult {
        self.metadata.validate()?;

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
