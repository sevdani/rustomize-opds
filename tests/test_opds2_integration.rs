//! Integration tests for OPDS 2.0 parser and generator

use rustomize_opds::opds2::{Generator, Parser};

#[test]
fn test_parse_minimal_opds2_feed() {
    let parser = Parser::new();

    // Read the minimal feed fixture
    let json = std::fs::read_to_string("tests/fixtures/opds2/minimal_feed.json")
        .expect("Failed to read fixture file");

    // Try parsing JSON first to verify
    let parsed_json: serde_json::Value =
        serde_json::from_str(&json).expect("Failed to parse JSON as Value");

    // Print to stderr
    eprintln!("Parsed JSON as Value: {}", parsed_json);
    eprintln!("Attempting to parse as Feed...");

    // Parse the feed
    let feed = parser.parse_feed(&json).expect("Failed to parse feed");

    // Verify feed metadata
    assert_eq!(feed.title, Some("Minimal OPDS 2.0 Feed".to_string()));
    assert!(feed.identifier.is_some());

    // Verify links
    assert!(!feed.feed_links.is_empty());
    assert_eq!(feed.feed_links[0].rel, "self");
    assert_eq!(
        feed.feed_links[0].href,
        "http://example.com/opds2/minimal.json"
    );

    // Verify publications collection exists
    assert!(feed.publications.is_some());
    let publications = feed.publications.as_ref().unwrap();
    assert!(!publications.is_empty());

    // Verify first publication
    let pub_ = &publications[0];
    assert_eq!(pub_.metadata.title, Some("Sample Publication".to_string()));
    assert!(pub_.metadata.type_.is_some());
}

#[test]
fn test_parse_full_opds2_feed() {
    let parser = Parser::new();

    // Read the full feed fixture
    let json = std::fs::read_to_string("tests/fixtures/opds2/full_feed.json")
        .expect("Failed to read fixture file");

    // Parse the feed
    let feed = parser.parse_feed(&json).expect("Failed to parse feed");

    // Verify feed metadata
    assert_eq!(feed.title, Some("Complete OPDS 2.0 Catalog".to_string()));
    assert!(feed.subtitle.is_some());
    assert_eq!(feed.items_per_page, Some(50));
    assert_eq!(feed.current_page, Some(1));
    assert_eq!(feed.number_of_items, Some(1000));

    // Verify multiple link types
    assert!(!feed.feed_links.is_empty());
    let self_link = feed
        .feed_links
        .iter()
        .find(|l| l.rel == "self")
        .expect("Should have self link");
    assert_eq!(self_link.href, "http://example.com/opds2/full.json");

    // Verify navigation collection
    assert!(feed.navigation.is_some());
    let navigation = feed.navigation.as_ref().unwrap();
    assert!(!navigation.is_empty());

    // Verify publications collection
    assert!(feed.publications.is_some());
    let publications = feed.publications.as_ref().unwrap();
    assert!(publications.len() >= 2);

    // Verify first publication has complex metadata
    let pub_ = &publications[0];
    assert_eq!(
        pub_.metadata.title,
        Some("The Art of Programming".to_string())
    );
    assert!(pub_.metadata.author.is_some());
    assert!(pub_.metadata.language.is_some());

    // Verify links with properties
    let acquisition_link = pub_
        .links
        .as_ref()
        .unwrap()
        .iter()
        .find(|l| l.rel == "acquisition")
        .expect("Should have acquisition link");

    assert!(acquisition_link.properties.is_some());
}

#[test]
fn test_validate_parsed_feed() {
    let parser = Parser::new();

    let json = std::fs::read_to_string("tests/fixtures/opds2/minimal_feed.json")
        .expect("Failed to read fixture file");

    let feed = parser.parse_feed(&json).expect("Failed to parse feed");

    // Validate the parsed feed
    let result = parser.validate_feed(&feed);
    assert!(result.is_ok());
}

#[test]
fn test_round_trip_generation() {
    let generator = Generator::new();
    let parser = Parser::new();

    let json = std::fs::read_to_string("tests/fixtures/opds2/minimal_feed.json")
        .expect("Failed to read fixture file");

    // Parse the original feed
    let feed = parser.parse_feed(&json).expect("Failed to parse feed");

    // Generate JSON from parsed feed
    let generated_json = generator
        .generate_and_validate_feed(&feed)
        .expect("Failed to generate JSON");

    // Parse the generated JSON
    let reparsed_feed = parser
        .parse_feed(&generated_json)
        .expect("Failed to reparse generated JSON");

    // Verify basic properties match
    assert_eq!(reparsed_feed.title, feed.title);
}

#[test]
fn test_generate_minimal_structures() {
    // Test generating minimal feed
    let feed = Generator::generate_minimal_feed("Test Catalog", "http://example.com/catalog.json");

    assert_eq!(feed.title, Some("Test Catalog".to_string()));
    assert_eq!(feed.feed_links.len(), 1);
    assert_eq!(feed.feed_links[0].rel, "self");
    assert_eq!(
        feed.feed_links[0].type_,
        Some("application/opds+json".to_string())
    );

    // Test generating minimal publication
    let publication = Generator::generate_minimal_publication(
        "Test Book",
        "http://example.com/book.json",
        "http://example.com/book.epub",
    );

    assert_eq!(publication.metadata.title, Some("Test Book".to_string()));
    assert_eq!(publication.links.as_ref().unwrap().len(), 2);

    let download_link = publication
        .links
        .as_ref()
        .unwrap()
        .iter()
        .find(|l| l.rel == "download")
        .expect("Should have download link");

    assert_eq!(download_link.href, "http://example.com/book.epub");
}
