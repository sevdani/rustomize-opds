//! OPDS Common Types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::error::{Validate, ValidationResult};

/// Common metadata fields
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CommonMetadata {
    /// Identifier for the feed or publication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Type (schema.org type)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Title of the feed or publication
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
}

impl CommonMetadata {
    /// Create new metadata with title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..Default::default()
        }
    }

    /// Validate the metadata
    pub fn validate(&self) -> ValidationResult {
        if self.title.is_none() || self.title.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            return Err(crate::common::error::OpdsError::MissingField(
                "title".to_string(),
            ));
        }
        Ok(())
    }
}

impl Validate for CommonMetadata {
    fn validate(&self) -> ValidationResult {
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_metadata_new() {
        let metadata = CommonMetadata::new("Test Title");
        assert_eq!(metadata.title, Some("Test Title".to_string()));
        assert!(metadata.identifier.is_none());
        assert!(metadata.description.is_none());
    }

    #[test]
    fn test_common_metadata_validate_success() {
        let metadata = CommonMetadata::new("Valid Title");
        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn test_common_metadata_validate_empty_title() {
        let metadata = CommonMetadata {
            title: Some("".to_string()),
            ..CommonMetadata::default()
        };
        assert!(metadata.validate().is_err());
    }

    #[test]
    fn test_common_metadata_validate_none_title() {
        let metadata = CommonMetadata {
            title: None,
            ..CommonMetadata::default()
        };
        assert!(metadata.validate().is_err());
    }

    #[test]
    fn test_common_metadata_all_fields() {
        let metadata = CommonMetadata {
            identifier: Some("urn:uuid:test".to_string()),
            type_: Some("http://schema.org/Book".to_string()),
            title: Some("Full Title".to_string()),
            subtitle: Some("Subtitle".to_string()),
            modified: Some(chrono::Utc::now()),
            description: Some("Description".to_string()),
            items_per_page: Some(10),
            current_page: Some(1),
            number_of_items: Some(100),
        };
        assert!(metadata.validate().is_ok());
    }
}
