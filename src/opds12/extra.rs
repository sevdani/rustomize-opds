//! OPDS 1.2 types -补充 FacetGroup

use serde::{Deserialize, Serialize};

/// Facet group (optional extension)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FacetGroup {
    /// Facet group name
    #[serde(rename = "$text")]
    pub name: String,
    #[serde(rename = "opds:facetGroup", skip_serializing_if = "Option::is_none")]
    pub opds_facet_group: Option<String>,
}
