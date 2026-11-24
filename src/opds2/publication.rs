//! OPDS 2.0 Publication types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::{Validate, ValidationResult};
use crate::opds2::types::{
    Accessibility, AltIdentifier, LanguageMap, Layout, ReadingProgression, Subject, Tdm,
};
use crate::opds2::{CollectionInfo, Contributor, Link, SeriesInfo};

/// OPDS 2.0 Publication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Publication {
    /// Publication metadata - required by OPDS 2.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PublicationMetadata>,
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
    /// Author (contributor object with links)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    /// Translator (contributor object with links)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translator: Option<Contributor>,
    /// Editor (contributor object with links)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<Contributor>,
    /// Illustrator (contributor object with links)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illustrator: Option<Contributor>,
    /// Narrator (contributor object with links)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrator: Option<Contributor>,
    /// Publisher (contributor object with links)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Contributor>,
    /// Artist (contributor object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<Contributor>,
    /// Letterer (contributor object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letterer: Option<Contributor>,
    /// Penciler (contributor object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penciler: Option<Contributor>,
    /// Colorist (contributor object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorist: Option<Contributor>,
    /// Inker (contributor object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inker: Option<Contributor>,
    /// Contributor (generic)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor: Option<Contributor>,
    /// Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Published date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,
    /// Series info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<SeriesInfo>,
    /// Collection info (full object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<CollectionInfo>,
    /// Belongs to (collection, journal, magazine, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belongs_to: Option<BelongsTo>,
    /// Contains (article, chapter, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<Contains>,
    /// Accessibility metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<Accessibility>,
    /// Layout hint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    /// Reading progression direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reading_progression: Option<ReadingProgression>,
    /// Duration (in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// Number of pages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_pages: Option<u32>,
    /// Subject terms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Vec<Subject>>,
    /// Conforms to (spec compliance URIs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conforms_to: Option<Vec<String>>,
    /// Alternative identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_identifier: Option<Vec<AltIdentifier>>,
    /// TDM (Text and Data Mining) metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdm: Option<Tdm>,
}

/// Belongs to structure (collection, journal, magazine, etc.)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct BelongsTo {
    /// Collection info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<CollectionInfo>,
    /// Journal info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal: Option<PeriodicalInfo>,
    /// Magazine info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magazine: Option<PeriodicalInfo>,
    /// Newspaper info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newspaper: Option<PeriodicalInfo>,
    /// Season info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<SeasonInfo>,
    /// Series info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<SeriesInfo>,
    /// Story arc info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_arc: Option<StoryArcInfo>,
    /// Volume info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeInfo>,
}

/// Periodical info (journal, magazine, newspaper)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeriodicalInfo {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl PeriodicalInfo {
    /// Create new periodical info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// Season info
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeasonInfo {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl SeasonInfo {
    /// Create new season info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// Story arc info
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryArcInfo {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl StoryArcInfo {
    /// Create new story arc info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// Volume info
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolumeInfo {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl VolumeInfo {
    /// Create new volume info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// Contains structure (article, chapter, etc.)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contains {
    /// Article info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article: Option<ArticleInfo>,
    /// Chapter info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<ChapterInfo>,
    /// Episode info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<EpisodeInfo>,
    /// Issue info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<IssueInfo>,
    /// Season info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<SeasonInfo>,
    /// Series info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<SeriesInfo>,
    /// Story arc info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_arc: Option<StoryArcInfo>,
    /// Volume info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<VolumeInfo>,
}

/// Article info
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticleInfo {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl ArticleInfo {
    /// Create new article info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// Chapter info
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterInfo {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl ChapterInfo {
    /// Create new chapter info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// Episode info
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpisodeInfo {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl EpisodeInfo {
    /// Create new episode info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
}

/// Issue info
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssueInfo {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl IssueInfo {
    /// Create new issue info with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }
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
            artist: None,
            letterer: None,
            penciler: None,
            colorist: None,
            inker: None,
            contributor: None,
            language: None,
            published: None,
            series: None,
            collection: None,
            belongs_to: None,
            contains: None,
            accessibility: None,
            layout: None,
            reading_progression: None,
            duration: None,
            number_of_pages: None,
            subject: None,
            conforms_to: None,
            alt_identifier: None,
            tdm: None,
        }
    }
}

impl Publication {
    /// Create a new publication
    pub fn new(metadata: Option<PublicationMetadata>, links: Option<Vec<Link>>) -> Self {
        Self {
            metadata,
            links,
            images: None,
            reading_order: None,
            toc: None,
            page_list: None,
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: PublicationMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Add images
    pub fn with_images(mut self, images: Vec<Link>) -> Self {
        self.images = Some(images);
        self
    }

    /// Validate the publication
    pub fn validate(&self) -> ValidationResult {
        // metadata is required by OPDS 2.0
        let metadata =
            self.metadata
                .as_ref()
                .ok_or(crate::common::error::OpdsError::Validation(
                    "Publication metadata is required".to_string(),
                ))?;

        if metadata.title.is_none()
            || metadata
                .title
                .as_ref()
                .and_then(|t| t.get_primary())
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
                    l.rel
                        .as_ref()
                        .map(|rels| {
                            rels.contains(&"acquisition".to_string())
                                || rels.contains(&"borrow".to_string())
                                || rels.contains(&"buy".to_string())
                                || rels.contains(&"download".to_string())
                                || rels.contains(&"preview".to_string())
                                || rels.contains(&"subscribe".to_string())
                                || rels
                                    .iter()
                                    .any(|r| r.starts_with("http://opds-spec.org/acquisition"))
                        })
                        .unwrap_or(false)
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
            title: Some(LanguageMap::new(title)),
            ..Default::default()
        }
    }
}

/// Author information (can include links)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    /// Author name - required by OPDS 2.0
    pub name: Option<String>,
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

impl Author {
    /// Create new author with name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// Set links on author
    pub fn with_links(mut self, links: Vec<Link>) -> Self {
        self.links = Some(links);
        self
    }
}

impl Validate for Author {
    fn validate(&self) -> ValidationResult {
        // If author has links, they must be valid
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

    #[test]
    fn test_series_info_new() {
        let series = SeriesInfo::new("Test Series");
        assert_eq!(series.name, Some("Test Series".to_string()));
        assert!(series.position.is_none());
    }

    #[test]
    fn test_series_info_default() {
        let series = SeriesInfo::default();
        assert!(series.name.is_none());
        assert!(series.position.is_none());
    }

    #[test]
    fn test_series_info_with_position() {
        let series = SeriesInfo::new("Test Series");
        assert_eq!(series.name, Some("Test Series".to_string()));
    }
}
