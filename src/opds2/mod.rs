//! OPDS 2.0 Module

mod feed;
mod generator;
mod parser;
mod publication;
mod types;

pub use feed::{Facet, Feed, Group};
pub use generator::{FeedWithPublications, PublicationWithLink};
pub use generator::{Generator, LinkWithProperties, LinkWithType};
pub use parser::Parser;
pub use publication::{
    ArticleInfo, ChapterInfo, EpisodeInfo, IssueInfo, PeriodicalInfo, SeasonInfo, StoryArcInfo,
    VolumeInfo,
};
pub use publication::{Author, Publication, PublicationMetadata};
pub use types::{
    Accessibility, AcquisitionObject, AltIdentifier, Availability, AvailabilityState,
    CollectionInfo, Contributor, Copies, FeedMetadata, Holds, LanguageMap, Layout, Link,
    LinkProperties, Price, ReadingProgression, SeriesInfo, Subject, Tdm,
};
