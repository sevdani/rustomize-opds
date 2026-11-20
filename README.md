# rustomize-opds

![CI Status](https://github.com/rustomize/rustomize-opds/workflows/Mainline%20CI/badge.svg)
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

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustomize-opds = "X.X.X"
```

## Quick Start

### OPDS 2.0 Parsing and Generation

```rust
use rustomize_opds::opds2::{Parser, Generator, Feed};
use rustomize_opds::common::CommonMetadata;

// Create a feed
let metadata = CommonMetadata::new("My OPDS Catalog");
let feed = Feed::new(metadata, vec![]);

// Generate to JSON
let generator = Generator::new();
let json = generator.generate_feed(&feed).unwrap();

// Parse from JSON
let parser = Parser::new();
let parsed_feed = parser.parse_feed(&json).unwrap();
```

### OPDS 1.2 Parsing and Generation

```rust
use rustomize_opds::opds12::{Parser, Generator};

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

- `common`: Shared types and utilities for both OPDS versions
- `opds12`: OPDS 1.2 (Atom/XML) parsing and generation
- `opds2`: OPDS 2.0 (JSON) parsing and generation

## Supported OPDS Features

### OPDS 2.0

- Feed structure: Metadata, links, navigation, publications collections
- Publication metadata: Title, author, language, identifier, modified date
- Links: Self, alternate, acquisition, thumbnail, cover
- Link properties: Price, availability, holds, copies, indirect acquisitions
- Pagination: Items per page, current page, number of items

### OPDS 1.2

- Feed structure: Title, author, updated, entries, links
- Entry structure: Title, author, summary, content, links
- Dublin Core extensions: Identifier, title, language, publisher, subject, description
- OPDS facets: Facet groups, active facets
- Acquisition feeds: Full support for OPDS acquisition links
- Price support: OPDS 1.2 price elements with currency codes

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
