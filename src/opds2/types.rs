//! OPDS 2.0 Link types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::{OpdsError, Result, Validate, ValidationResult};

/// Language map for multilingual content
///
/// Supports both simple string and language-map format:
/// ```json
/// {"en": "Title", "fr": "Titre"}
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LanguageMap {
    Simple(String),
    Map(HashMap<String, String>),
}

impl LanguageMap {
    /// Create a new language map from a simple string
    pub fn new(s: impl Into<String>) -> Self {
        Self::Simple(s.into())
    }

    /// Create a new language map from a HashMap
    pub fn from_map(map: HashMap<String, String>) -> Self {
        Self::Map(map)
    }

    /// Get the primary language value (first in map or the simple string)
    pub fn get_primary(&self) -> Option<String> {
        match self {
            LanguageMap::Simple(s) => Some(s.clone()),
            LanguageMap::Map(m) => m.values().next().cloned(),
        }
    }
}

/// Link object for OPDS 2.0
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Link {
    /// Link relation (string or array of strings)
    #[serde(deserialize_with = "deserialize_rel")]
    #[serde(serialize_with = "serialize_rel")]
    pub rel: Option<Vec<String>>,
    /// Link target (URI) - required by OPDS 2.0
    pub href: Option<String>,
    /// Media type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Link title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Height (for images)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Width (for images)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Is this a template URL?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templated: Option<bool>,
    /// Link properties (price, availability, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<LinkProperties>,
}

/// Deserialize rel field that can be a string or array of strings
fn deserialize_rel<'de, D>(deserializer: D) -> std::result::Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(deserializer)?;
    match opt {
        Some(serde_json::Value::String(s)) => Ok(Some(vec![s])),
        Some(serde_json::Value::Array(arr)) => {
            let rels: Vec<String> = arr
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            if rels.is_empty() {
                Ok(None)
            } else {
                Ok(Some(rels))
            }
        }
        Some(v) => Err(serde::de::Error::custom(format!(
            "Expected string or array for 'rel', got: {}",
            v
        ))),
        None => Ok(None),
    }
}

/// Serialize rel field as string or array of strings
fn serialize_rel<S>(
    value: &Option<Vec<String>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;

    match value {
        Some(vec) if vec.len() == 1 => serializer.serialize_str(&vec[0]),
        Some(vec) => {
            let mut seq = serializer.serialize_seq(Some(vec.len()))?;
            for item in vec {
                seq.serialize_element(item)?;
            }
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}

/// Link properties (from OPDS 2.0 spec)
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct LinkProperties {
    /// Price information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    /// Indirect acquisition chain
    #[serde(
        rename = "indirectAcquisition",
        skip_serializing_if = "Option::is_none"
    )]
    pub indirect_acquisition: Option<Vec<AcquisitionObject>>,
    /// Number of items hint
    #[serde(rename = "numberOfItems", skip_serializing_if = "Option::is_none")]
    pub number_of_items: Option<u32>,
    /// Availability information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    /// Holds information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holds: Option<Holds>,
    /// Copies information
    #[serde(rename = "copies", skip_serializing_if = "Option::is_none")]
    pub copies: Option<Copies>,
}

/// Price object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Price {
    /// Currency code (ISO 4217)
    pub currency: Option<String>,
    /// Price value
    pub value: Option<f64>,
}

impl Default for Price {
    fn default() -> Self {
        Self {
            currency: Some("USD".to_string()),
            value: Some(0.0),
        }
    }
}

/// Acquisition object for indirect acquisitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AcquisitionObject {
    /// Media type
    pub r#type: Option<String>,
    /// Child acquisition objects (nested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child: Option<Vec<AcquisitionObject>>,
}

impl Default for AcquisitionObject {
    fn default() -> Self {
        Self {
            r#type: Some("application/epub+zip".to_string()),
            child: None,
        }
    }
}

/// Availability state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AvailabilityState {
    /// Available
    Available,
    /// Unavailable
    Unavailable,
    /// Reserved
    Reserved,
    /// Ready
    Ready,
}

/// Availability information
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Availability {
    /// State of availability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<AvailabilityState>,
    /// Timestamp since state change
    #[serde(rename = "since", skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Timestamp until state change
    #[serde(rename = "until", skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

impl Availability {
    /// Create a new availability with required state
    pub fn new(state: AvailabilityState) -> Self {
        Self {
            state: Some(state),
            since: None,
            until: None,
        }
    }

    /// Validate availability
    pub fn validate(&self) -> Result<()> {
        // state is required when Availability is present
        if self.state.is_none() {
            return Err(OpdsError::Validation(
                "Availability state is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Holds information (library feature)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Holds {
    /// Total holds
    #[serde(rename = "total")]
    pub total: Option<u32>,
    /// Position in hold list
    #[serde(rename = "position")]
    pub position: Option<u32>,
}

impl Holds {
    /// Create a new holds object
    pub fn new(total: u32, position: u32) -> Self {
        Self {
            total: Some(total),
            position: Some(position),
        }
    }
}

/// Copies information (library feature)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Copies {
    /// Total copies
    #[serde(rename = "total")]
    pub total: Option<u32>,
    /// Available copies
    #[serde(rename = "available")]
    pub available: Option<u32>,
}

impl Copies {
    /// Create a new copies object
    pub fn new(total: u32, available: u32) -> Self {
        Self {
            total: Some(total),
            available: Some(available),
        }
    }
}

impl Link {
    /// Create a new link with required href
    pub fn new(href: impl Into<String>) -> Self {
        Self {
            rel: None,
            href: Some(href.into()),
            ..Default::default()
        }
    }

    /// Set the link relation
    pub fn with_rel(mut self, rel: impl Into<String>) -> Self {
        self.rel = Some(vec![rel.into()]);
        self
    }

    /// Set the link relation as an array
    pub fn with_rels(mut self, rels: Vec<impl Into<String>>) -> Self {
        self.rel = Some(rels.into_iter().map(|r| r.into()).collect());
        self
    }

    /// Validate the link
    pub fn validate(&self) -> Result<()> {
        // href is required by OPDS 2.0 - must be Some
        let href = self
            .href
            .as_ref()
            .ok_or(OpdsError::Validation("href is required".to_string()))?;

        if href.is_empty() {
            return Err(OpdsError::Validation("href cannot be empty".to_string()));
        }

        // Validate properties if present
        if let Some(ref properties) = self.properties {
            properties.validate()?;
        }

        Ok(())
    }
}

impl LinkProperties {
    /// Validate link properties
    pub fn validate(&self) -> Result<()> {
        if let Some(price) = &self.price {
            let value = price
                .value
                .as_ref()
                .ok_or(OpdsError::Validation("price value is required".to_string()))?;
            if *value < 0.0 {
                return Err(OpdsError::Validation(
                    "price value cannot be negative".to_string(),
                ));
            }
        }

        // Validate availability if present
        if let Some(ref availability) = self.availability {
            availability.validate()?;
        }

        Ok(())
    }
}

impl Validate for LinkProperties {
    fn validate(&self) -> Result<()> {
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_new() {
        let link = Link::new("http://example.com");
        assert!(link.rel.is_none());
        assert_eq!(link.href, Some("http://example.com".to_string()));
        assert!(link.type_.is_none());
    }

    #[test]
    fn test_link_validation_success() {
        let link = Link::new("http://example.com");
        assert!(link.validate().is_ok());
    }

    #[test]
    fn test_link_validation_empty_href() {
        let link = Link {
            rel: None,
            href: Some("".to_string()),
            ..Default::default()
        };
        assert!(link.validate().is_err());
    }

    #[test]
    fn test_link_validation_missing_href() {
        let link = Link {
            rel: None,
            href: None,
            ..Default::default()
        };
        assert!(link.validate().is_err());
    }

    #[test]
    fn test_link_with_all_fields() {
        let link = Link {
            rel: Some(vec!["alternate".to_string()]),
            href: Some("http://example.com/epub".to_string()),
            type_: Some("application/epub+zip".to_string()),
            title: Some("EPUB Version".to_string()),
            height: Some(100),
            width: Some(200),
            templated: Some(false),
            properties: None,
        };
        assert!(link.validate().is_ok());
    }

    #[test]
    fn test_link_properties_default() {
        let props = LinkProperties::default();
        assert!(props.price.is_none());
        assert!(props.indirect_acquisition.is_none());
        assert!(props.holds.is_none());
        assert!(props.copies.is_none());
    }

    #[test]
    fn test_holds_new() {
        let holds = Holds::new(10, 5);
        assert_eq!(holds.total, Some(10));
        assert_eq!(holds.position, Some(5));
    }

    #[test]
    fn test_holds_default() {
        let holds = Holds::default();
        assert!(holds.total.is_none());
        assert!(holds.position.is_none());
    }

    #[test]
    fn test_holds_with_all_fields() {
        let holds = Holds::new(10, 5);
        assert_eq!(holds.total, Some(10));
        assert_eq!(holds.position, Some(5));
    }

    #[test]
    fn test_copies_new() {
        let copies = Copies::new(100, 50);
        assert_eq!(copies.total, Some(100));
        assert_eq!(copies.available, Some(50));
    }

    #[test]
    fn test_copies_default() {
        let copies = Copies::default();
        assert!(copies.total.is_none());
        assert!(copies.available.is_none());
    }

    #[test]
    fn test_copies_with_all_fields() {
        let copies = Copies::new(100, 50);
        assert_eq!(copies.total, Some(100));
        assert_eq!(copies.available, Some(50));
    }

    #[test]
    fn test_link_properties_validation_positive_price() {
        let props = LinkProperties {
            price: Some(Price {
                currency: Some("USD".to_string()),
                value: Some(10.99),
            }),
            ..LinkProperties::default()
        };
        assert!(props.validate().is_ok());
    }

    #[test]
    fn test_link_properties_validation_negative_price() {
        let props = LinkProperties {
            price: Some(Price {
                currency: Some("USD".to_string()),
                value: Some(-1.00),
            }),
            ..LinkProperties::default()
        };
        assert!(props.validate().is_err());
    }

    #[test]
    fn test_price_default() {
        let price = Price::default();
        assert_eq!(price.currency, Some("USD".to_string()));
        assert_eq!(price.value, Some(0.0));
    }

    #[test]
    fn test_acquisition_object_default() {
        let obj = AcquisitionObject::default();
        assert_eq!(obj.r#type, Some("application/epub+zip".to_string()));
        assert!(obj.child.is_none());
    }

    #[test]
    fn test_availability_default() {
        let avail = Availability::default();
        assert!(avail.state.is_none());
        assert!(avail.since.is_none());
        assert!(avail.until.is_none());
    }

    #[test]
    fn test_availability_new() {
        let avail = Availability::new(AvailabilityState::Available);
        assert!(matches!(avail.state, Some(AvailabilityState::Available)));
        assert!(avail.since.is_none());
        assert!(avail.until.is_none());
    }

    #[test]
    fn test_validation_fails_on_empty_link_href() {
        let link = Link {
            href: Some("".to_string()),
            ..Link::default()
        };

        let result = link.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }
}

/// Feed metadata structure (nested in Feed per spec)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedMetadata {
    /// Feed identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Type (schema.org type)
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Feed title - required by OPDS 2.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<LanguageMap>,
    /// Subtitle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<LanguageMap>,
    /// Last modification date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Items per page (pagination)
    #[serde(rename = "itemsPerPage", skip_serializing_if = "Option::is_none")]
    pub items_per_page: Option<u32>,
    /// Current page number (pagination)
    #[serde(rename = "currentPage", skip_serializing_if = "Option::is_none")]
    pub current_page: Option<u32>,
    /// Total number of items (pagination)
    #[serde(rename = "numberOfItems", skip_serializing_if = "Option::is_none")]
    pub number_of_items: Option<u32>,
}

impl FeedMetadata {
    /// Create new feed metadata with required title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(LanguageMap::new(title)),
            ..Default::default()
        }
    }

    /// Validate feed metadata
    pub fn validate(&self) -> ValidationResult {
        if self.title.is_none()
            || self
                .title
                .as_ref()
                .and_then(|t| t.get_primary())
                .map(|s| s.is_empty())
                .unwrap_or(true)
        {
            return Err(OpdsError::MissingField("title".to_string()));
        }
        Ok(())
    }
}

impl Validate for FeedMetadata {
    fn validate(&self) -> ValidationResult {
        self.validate()
    }
}

/// Contributor object (for author, translator, editor, etc.)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contributor {
    /// Contributor name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier (URI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Sort order
    #[serde(rename = "sortAs", skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<String>,
    /// Links to contributor resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl Contributor {
    /// Create new contributor with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// Set links on contributor
    pub fn with_links(mut self, links: Vec<Link>) -> Self {
        self.links = Some(links);
        self
    }

    /// Validate contributor object
    pub fn validate(&self) -> ValidationResult {
        // If contributor has links, they must be valid
        if let Some(ref links) = self.links {
            for link in links {
                link.validate()?;
            }
        }
        Ok(())
    }
}

impl Validate for Contributor {
    fn validate(&self) -> ValidationResult {
        self.validate()
    }
}

/// Accessibility metadata
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Accessibility {
    /// Accessibility modes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<Vec<String>>,
    /// Access mode sufficient
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_mode_sufficient: Option<Vec<String>>,
    /// Accessibility features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_feature: Option<Vec<String>>,
    /// Accessibility hazards
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_hazard: Option<Vec<String>>,
    /// Accessibility summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_summary: Option<String>,
}

/// Subject object
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subject {
    /// Subject name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Subject scheme (URI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// Subject code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_as: Option<String>,
}

impl Subject {
    /// Create new subject with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// Collection information
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollectionInfo {
    /// Collection name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Position in collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl CollectionInfo {
    /// Create new collection info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// Series information
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesInfo {
    /// Series name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Position in series
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl SeriesInfo {
    /// Create new series info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// TDM (Text and Data Mining) metadata
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tdm {
    /// Reservation policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<String>,
    /// Policy URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

/// Alternative identifier
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct AltIdentifier {
    /// Identifier type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Identifier value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl AltIdentifier {
    /// Create new alt identifier
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: Some(value.into()),
            ..Default::default()
        }
    }
}

/// Layout enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    /// Fixed layout
    Fixed,
    /// Reflowable layout
    Reflowable,
    /// Scrolled layout
    Scrolled,
}

/// Reading progression enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReadingProgression {
    /// Right to left
    Rtl,
    /// Left to right
    Ltr,
}
