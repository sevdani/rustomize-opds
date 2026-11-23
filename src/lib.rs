//! rustomize-opds - OPDS Parser and Generator Library
//!
//! A Rust library for parsing and generating OPDS (Open Publication Distribution System) feeds
//! for both OPDS 1.2 (Atom/XML) and OPDS 2.0 (JSON).
//!
//! # Overview
//!
//! OPDS (Open Publication Distribution System) is a family of specifications for distributing
//! electronic publications. This library supports:
//!
//! - **OPDS 1.2**: Atom-based XML format with OPDS and Dublin Core namespaces
//! - **OPDS 2.0**: JSON-based format based on Readium Web Publication Manifest
//!
//! # Examples
//!
//! ## OPDS 2.0 Parsing and Generation
//!
//! ```no_run
//! use rustomize_opds::opds2::{Parser, Generator, Feed};
//!
//! // Create a feed
//! let feed = Feed::new("My OPDS Catalog", vec![]);
//!
//! // Generate to JSON (with auto-validation)
//! let generator = Generator::new();
//! let json = generator.generate_and_validate_feed(&feed).unwrap();
//!
//! // Parse from JSON
//! let parser = Parser::new();
//! let parsed_feed = parser.parse_feed(&json).unwrap();
//! ```
//!
//! ## OPDS 1.2 Parsing and Generation
//!
//! ```no_run
//! use rustomize_opds::opds12::{Parser, Generator};
//!
//! // Generate a minimal feed
//! let feed = Generator::generate_minimal_feed(
//!     "urn:uuid:12345",
//!     "My OPDS Catalog",
//!     "http://example.com/opds"
//! );
//!
//! // Generate to XML (with auto-validation)
//! let generator = Generator::new();
//! let xml = generator.generate_and_validate_feed(&feed).unwrap();
//!
//! // Parse from XML
//! let parser = Parser::new();
//! let parsed_feed = parser.parse_feed(&xml).unwrap();
//! ```

pub mod common;
pub mod opds12;
pub mod opds2;

pub use common::{is_valid_currency, is_valid_image_format, OpdsError, Result};
pub use opds12::{Generator as Opds12Generator, Parser as Opds12Parser};
pub use opds2::{Generator as Opds2Generator, Parser as Opds2Parser};
