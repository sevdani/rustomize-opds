# rustomize-opds

[![Mainline CI](https://github.com/sevdani/rustomize-opds/actions/workflows/mainline.yml/badge.svg?branch=main)](https://github.com/sevdani/rustomize-opds/actions/workflows/mainline.yml)
[![Crates.io](https://img.shields.io/crates/v/rustomize-opds)](https://crates.io/crates/rustomize-opds)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/rustomize/rustomize-opds/blob/main/LICENSE)

A Rust library for parsing and generating OPDS (Open Publication Distribution System) feeds for both [OPDS 1.2](https://specs.opds.io/opds-1.2) (Atom/XML) and [OPDS 2.0](https://specs.opds.io/opds-2.0) (JSON).

## Overview

OPDS (Open Publication Distribution System) is a family of specifications for distributing electronic publications. This library provides:

- **OPDS 1.2 support**: Full Atom/XML format with OPDS and Dublin Core namespaces
- **OPDS 2.0 support**: JSON-based format based on Readium Web Publication Manifest
- **Type-safe parsing and generation**: Using `serde` for robust serialization
- **Validation**: Built-in validation for OPDS structures
- **Comprehensive tests**: Integration tests ensure correctness

## Features

- Parse OPDS 1.2 feeds from XML
- Generate OPDS 1.2 feeds to XML
- Parse OPDS 2.0 feeds from JSON
- Generate OPDS 2.0 feeds to JSON
- Full support for OPDS catalogs, navigation, and publications
- Support for prices, availability, holds, and copies
- Comprehensive validation
- Full pagination support
- OpenSearch search support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustomize-opds = "X.X.X"
```

## Quick Start

### OPDS 2.0 Parsing and Generation

```rust
use rustomize_opds::opds2::{Parser, Generator, Feed, FeedMetadata};

// Create a feed with metadata
let feed_metadata = FeedMetadata::new("My OPDS Catalog");
let self_link = rustomize_opds::opds2::Link::new("http://example.com/feed.json")
    .with_type("application/opds+json")
    .with_rel("self");

let feed = Feed::new_with_metadata(feed_metadata, vec![self_link]);

// Add publications collection
let publication = Generator::generate_minimal_publication(
    "My Book",
    "http://example.com/book.json",
    "http://example.com/book.epub",
);
let feed = feed.with_publications(vec![publication]);

// Generate to JSON
let generator = Generator::new();
let json = generator.generate_and_validate_feed(&feed).unwrap();

// Parse from JSON
let parser = Parser::new();
let parsed_feed = parser.parse_and_validate_feed(&json).unwrap();
```

### OPDS 1.2 Parsing and Generation

```rust
use rustomize_opds::opds12::{Parser, Generator, Feed, Entry, Link, Summary, Price};

// Generate a minimal feed
let feed = Generator::generate_minimal_feed(
    "urn:uuid:12345",
    "My OPDS Catalog",
    "http://example.com/opds"
);

// Generate to XML
let generator = Generator::new();
let xml = generator.generate_feed(&feed).unwrap();

// Parse from XML
let parser = Parser::new();
let parsed_feed = parser.parse_feed(&xml).unwrap();
```

## Modules

- `common`: Shared utilities and errors for both OPDS versions
- `opds12`: OPDS 1.2 (Atom/XML) parsing and generation
- `opds2`: OPDS 2.0 (JSON) parsing and generation

## Supported OPDS Features

### OPDS 2.0

- **Feed structure**: Nested metadata, links, navigation, publications, groups collections
- **Publication metadata**: Title, author, translator, editor, illustrator, narrator, publisher, language, identifier, modified date
- **Contributor objects**: Full contributor objects with links for all contributor types
- **Links**: Self, alternate, acquisition, thumbnail, cover with properties
- **Link properties**: Price, availability, holds, copies, indirect acquisitions
- **Link rel as array**: Support for single rel string or array of rels
- **Accessibility metadata**: Access modes, features, and summaries
- **Layout properties**: Fixed, reflowable, scrolled layouts
- **Pagination**: Items per page, current page, number of items
- **BelongsTo structure**: Collection, journal, magazine, newspaper, season, series, story arc, volume
- **Contains structure**: Article, chapter, episode, issue, season, series, story arc, volume
- **Subject terms**: Tagging with schemes and codes
- **TDM metadata**: Text and data mining policies

### OPDS 1.2

- **Feed structure**: Title, author, updated, entries, links, id, rights, icon, logo, generator
- **Entry structure**: Title, author, summary, content, links, id, published, contributor, rights
- **Dublin Core extensions**: Identifier, title, language, issued, publisher, subject, description, creator, rights
- **OPDS facets**: Facet groups, active facets with `opds_facet_group` and `opds_active_facet` attributes
- **Acquisition feeds**: Full support for OPDS acquisition links with 6 relation types
- **Price support**: OPDS 1.2 price elements with ISO 4217 currency codes
- **Indirect acquisition**: Nested acquisition chains for complex workflows (containers, payment flows)
- **Search support**: OpenSearch description documents with `search` relation
- **Pagination**: Partial vs Complete Acquisition Feeds with `fh:complete` element
- **Catalog aggregation**: `atom:source` element support

## Error Handling

The library uses `thiserror` for comprehensive error types:

- `Xml`: XML parsing errors
- `Json`: JSON parsing errors
- `Validation`: OPDS validation errors
- `MissingField`: Missing required fields
- `InvalidMediaType`: Invalid media types
- `InvalidCurrency`: Invalid currency codes
- `Io`: IO errors

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


## References

- [OPDS 2.0 Specification](https://specs.opds.io/opds-2.0)
- [OPDS 1.2 Specification](https://specs.opds.io/opds-1.2)
