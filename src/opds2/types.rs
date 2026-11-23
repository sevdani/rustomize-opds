//! OPDS 2.0 Link types

use serde::{Deserialize, Serialize};

use crate::common::{OpdsError, Result, Validate};

/// Link object for OPDS 2.0
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Link {
    /// Link relation
    pub rel: String,
    /// Link target (URI)
    pub href: String,
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
    pub currency: String,
    /// Price value
    pub value: f64,
}

impl Default for Price {
    fn default() -> Self {
        Self {
            currency: "USD".to_string(),
            value: 0.0,
        }
    }
}

/// Acquisition object for indirect acquisitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AcquisitionObject {
    /// Media type
    pub r#type: String,
    /// Child acquisition objects (nested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child: Option<Vec<AcquisitionObject>>,
}

impl Default for AcquisitionObject {
    fn default() -> Self {
        Self {
            r#type: "application/epub+zip".to_string(),
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Availability {
    /// State of availability
    pub state: AvailabilityState,
    /// Timestamp since state change
    #[serde(rename = "since", skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Timestamp until state change
    #[serde(rename = "until", skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

impl Default for Availability {
    fn default() -> Self {
        Self {
            state: AvailabilityState::Available,
            since: None,
            until: None,
        }
    }
}

/// Holds information (library feature)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Holds {
    /// Total holds
    #[serde(rename = "total")]
    pub total: u32,
    /// Position in hold list
    #[serde(rename = "position")]
    pub position: u32,
}

/// Copies information (library feature)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Copies {
    /// Total copies
    #[serde(rename = "total")]
    pub total: u32,
    /// Available copies
    #[serde(rename = "available")]
    pub available: u32,
}

impl Link {
    /// Create a new link
    pub fn new(rel: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            rel: rel.into(),
            href: href.into(),
            ..Default::default()
        }
    }

    /// Validate the link
    pub fn validate(&self) -> Result<()> {
        if self.href.is_empty() {
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
            if price.value < 0.0 {
                return Err(OpdsError::Validation(
                    "price value cannot be negative".to_string(),
                ));
            }
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
        let link = Link::new("self", "http://example.com");
        assert_eq!(link.rel, "self");
        assert_eq!(link.href, "http://example.com");
        assert!(link.type_.is_none());
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
    fn test_link_with_all_fields() {
        let link = Link {
            rel: "alternate".to_string(),
            href: "http://example.com/epub".to_string(),
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
    }

    #[test]
    fn test_link_properties_validation_positive_price() {
        let props = LinkProperties {
            price: Some(Price {
                currency: "USD".to_string(),
                value: 10.99,
            }),
            ..LinkProperties::default()
        };
        assert!(props.validate().is_ok());
    }

    #[test]
    fn test_link_properties_validation_negative_price() {
        let props = LinkProperties {
            price: Some(Price {
                currency: "USD".to_string(),
                value: -1.00,
            }),
            ..LinkProperties::default()
        };
        assert!(props.validate().is_err());
    }

    #[test]
    fn test_price_default() {
        let price = Price::default();
        assert_eq!(price.currency, "USD");
        assert_eq!(price.value, 0.0);
    }

    #[test]
    fn test_acquisition_object_default() {
        let obj = AcquisitionObject::default();
        assert_eq!(obj.r#type, "application/epub+zip");
        assert!(obj.child.is_none());
    }

    #[test]
    fn test_availability_default() {
        let avail = Availability::default();
        assert!(matches!(avail.state, AvailabilityState::Available));
        assert!(avail.since.is_none());
        assert!(avail.until.is_none());
    }

    #[test]
    fn test_holds_default() {
        let holds = Holds::default();
        assert_eq!(holds.total, 0);
        assert_eq!(holds.position, 0);
    }

    #[test]
    fn test_copies_default() {
        let copies = Copies::default();
        assert_eq!(copies.total, 0);
        assert_eq!(copies.available, 0);
    }
}
