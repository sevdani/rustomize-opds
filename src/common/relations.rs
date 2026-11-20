//! OPDS Link Relations
//!
//! This module defines all standard OPDS link relations for both v1.2 and v2.0

/// Standard link relations from OPDS 1.2
pub mod opds12 {
    /// Acquisition relations from OPDS 1.2
    pub const ACQUISITION: &str = "http://opds-spec.org/acquisition";
    pub const ACQUISITION_OPEN_ACCESS: &str = "http://opds-spec.org/acquisition/open-access";
    pub const ACQUISITION_BORROW: &str = "http://opds-spec.org/acquisition/borrow";
    pub const ACQUISITION_BUY: &str = "http://opds-spec.org/acquisition/buy";
    pub const ACQUISITION_SAMPLE: &str = "http://opds-spec.org/acquisition/sample";
    pub const ACQUISITION_SUBSCRIBE: &str = "http://opds-spec.org/acquisition/subscribe";

    /// Image relations
    pub const IMAGE: &str = "http://opds-spec.org/image";
    pub const IMAGE_THUMBNAIL: &str = "http://opds-spec.org/image/thumbnail";

    /// Navigation relations
    pub const SORT_NEW: &str = "http://opds-spec.org/sort/new";
    pub const SORT_POPULAR: &str = "http://opds-spec.org/sort/popular";
    pub const FEATURED: &str = "http://opds-spec.org/featured";
    pub const RECOMMENDED: &str = "http://opds-spec.org/recommended";
    pub const SHLF: &str = "http://opds-spec.org/shelf";
    pub const SUBSCRIPTIONS: &str = "http://opds-spec.org/subscriptions";

    /// Catalog relations
    pub const START: &str = "start";
    pub const SELF: &str = "self";
    pub const UP: &str = "up";
    pub const SUBSECTION: &str = "subsection";
    pub const COLLECTION: &str = "collection";
    pub const CRAWLABLE: &str = "http://opds-spec.org/crawlable";

    /// Facet relations
    pub const FACET: &str = "http://opds-spec.org/facet";
}

/// Standard link relations from OPDS 2.0
pub mod opds2 {
    /// Acquisition relations from OPDS 2.0
    pub const ACQUISITION: &str = "acquisition";
    pub const BORROW: &str = "borrow";
    pub const BUY: &str = "buy";
    pub const DOWNLOAD: &str = "download";
    pub const PREVIEW: &str = "preview";
    pub const SUBSCRIBE: &str = "subscribe";

    /// Navigation relations
    pub const SELF: &str = "self";
    pub const START: &str = "start";
    pub const NEXT: &str = "next";
    pub const PREVIOUS: &str = "previous";
    pub const FIRST: &str = "first";
    pub const LAST: &str = "last";

    /// Search relation
    pub const SEARCH: &str = "search";

    /// Collection role names (used in OPDS 2.0 feeds)
    pub const NAVIGATION: &str = "navigation";
    pub const PUBLICATIONS: &str = "publications";
    pub const IMAGES: &str = "images";
    pub const FACETS: &str = "facets";
    pub const GROUPS: &str = "groups";
}
