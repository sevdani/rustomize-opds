//! Integration tests for OPDS 1.2 generator and parser round-trip validation

use rustomize_opds::opds12::{Author, Entry, Feed, Generator, Link, LinkWithProperties, Parser};

#[test]
fn test_generate_parse_minimal_feed() {
    let generator = Generator::new();

    // Generate a minimal feed
    let original_feed = Generator::generate_minimal_feed(
        "urn:uuid:generated-123",
        "Generated Test Feed",
        "http://example.com/feed.xml",
    );

    // Generate to XML
    let xml = generator
        .generate_feed(&original_feed)
        .expect("Failed to generate XML");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&xml).expect("Failed to parse XML");

    // Validate basic fields
    assert_eq!(parsed_feed.id, original_feed.id);
    assert_eq!(parsed_feed.title, original_feed.title);

    // Validate links
    assert_eq!(parsed_feed.links.len(), original_feed.links.len());
    assert_eq!(parsed_feed.links[0].rel, original_feed.links[0].rel);
    assert_eq!(parsed_feed.links[0].href, original_feed.links[0].href);

    // Validate parsed feed
    parser
        .validate_feed(&parsed_feed)
        .expect("Parsed feed validation failed");
}

#[test]
fn test_generate_parse_complex_feed_with_authors() {
    let generator = Generator::new();

    // Create feed with authors
    let mut feed = Feed::new("urn:uuid:complex-456", "Complex Feed");

    // Add authors
    feed.authors.push(Author {
        name: "John Doe".to_string(),
        uri: Some("http://example.com/john".to_string()),
        email: Some("john@example.com".to_string()),
    });
    feed.authors.push(Author {
        name: "Jane Smith".to_string(),
        uri: None,
        email: Some("jane@example.com".to_string()),
    });

    // Add links
    feed.links.push(
        Link::new("self", "http://example.com/feed.xml")
            .with_type("application/atom+xml;profile=opds-catalog;kind=navigation"),
    );
    feed.links
        .push(Link::new("start", "http://example.com/").with_type("text/html"));

    // Add entry
    let mut entry = Entry::new("urn:uuid:entry-789", "Test Book");
    entry.authors.push(Author {
        name: "Book Author".to_string(),
        uri: None,
        email: None,
    });
    // Summary type validation is tested in unit tests
    entry.links.push(
        Link::new(
            "http://opds-spec.org/acquisition",
            "http://example.com/book.epub",
        )
        .with_type("application/epub+zip"),
    );
    feed.entries.push(entry);

    // Generate to XML
    let xml = generator
        .generate_feed(&feed)
        .expect("Failed to generate XML");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&xml).expect("Failed to parse XML");

    // Validate authors
    assert_eq!(parsed_feed.authors.len(), 2);
    assert_eq!(parsed_feed.authors[0].name, "John Doe");
    assert_eq!(
        parsed_feed.authors[0].uri,
        Some("http://example.com/john".to_string())
    );
    assert_eq!(
        parsed_feed.authors[0].email,
        Some("john@example.com".to_string())
    );

    // Validate links
    assert_eq!(parsed_feed.links.len(), 2);

    // Validate entries
    assert_eq!(parsed_feed.entries.len(), 1);
    let parsed_entry = &parsed_feed.entries[0];
    assert_eq!(parsed_entry.title, "Test Book");
    assert_eq!(parsed_entry.authors.len(), 1);

    // Validate parsed feed
    parser
        .validate_feed(&parsed_feed)
        .expect("Parsed feed validation failed");
}

#[test]
fn test_generate_parse_feed_with_nested_indirect_acquisition() {
    let generator = Generator::new();

    let mut feed = Feed::new("urn:uuid:nested-101", "Nested Acquisition Feed");

    // Add entry with nested indirect acquisition
    let mut entry = Entry::new("urn:uuid:entry-nested-202", "Book with Nested Acq");
    let mut link = Link::new(
        "http://opds-spec.org/acquisition",
        "http://example.com/book.pdf",
    )
    .with_type("application/pdf");

    // Add nested indirect acquisition (Adobe DRM chain)
    let nested_acq = Generator::nested_indirect_acquisition(
        "application/vnd.adobe.adept+xml",
        "application/pdf",
    );
    link = link.with_indirect_acquisition(nested_acq);

    entry.links.push(link);
    feed.entries.push(entry);

    // Generate to XML
    let xml = generator
        .generate_feed(&feed)
        .expect("Failed to generate XML");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&xml).expect("Failed to parse XML");

    // Validate nested indirect acquisition
    let parsed_entry = &parsed_feed.entries[0];
    let parsed_link = &parsed_entry.links[0];

    assert!(parsed_link.indirect_acquisition.is_some());
    let acquisitions = parsed_link.indirect_acquisition.as_ref().unwrap();
    assert_eq!(acquisitions.len(), 1);

    let outer_acq = &acquisitions[0];
    assert_eq!(outer_acq.r#type, "application/vnd.adobe.adept+xml");
    assert!(outer_acq.child.is_some());

    let inner_acq = outer_acq.child.as_ref().unwrap().first().unwrap();
    assert_eq!(inner_acq.r#type, "application/pdf");

    // Validate parsed feed
    parser
        .validate_feed(&parsed_feed)
        .expect("Parsed feed validation failed");
}

#[test]
fn test_generate_parse_feed_with_multiple_entries() {
    let generator = Generator::new();

    let mut feed = Feed::new("urn:uuid:multi-303", "Multiple Entries Feed");

    // Add multiple entries
    for i in 0..3 {
        let mut entry = Entry::new(format!("urn:uuid:entry-{}-404", i), format!("Book {}", i));

        // Add different link types
        let link_type = if i == 0 {
            "http://opds-spec.org/acquisition/open-access"
        } else {
            "http://opds-spec.org/acquisition/borrow"
        };

        entry.links.push(
            Link::new(link_type, format!("http://example.com/book{}.epub", i))
                .with_type("application/epub+zip"),
        );
        feed.entries.push(entry);
    }

    // Generate to XML
    let xml = generator
        .generate_feed(&feed)
        .expect("Failed to generate XML");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&xml).expect("Failed to parse XML");

    // Validate entries count
    assert_eq!(parsed_feed.entries.len(), 3);

    // Validate each entry
    for i in 0..3 {
        let entry = &parsed_feed.entries[i as usize];
        assert_eq!(entry.title, format!("Book {}", i));
        assert_eq!(entry.id, format!("urn:uuid:entry-{}-404", i));
    }

    // Validate parsed feed
    parser
        .validate_feed(&parsed_feed)
        .expect("Parsed feed validation failed");
}

#[test]
fn test_validation_fails_on_empty_id() {
    let generator = Generator::new();

    // Create feed with empty ID (invalid)
    let mut feed = Feed::new("", "Test Feed");
    let mut entry = Entry::new("urn:uuid:test-505", "Test Entry");
    entry.links.push(
        Link::new(
            "http://opds-spec.org/acquisition",
            "http://example.com/book.epub",
        )
        .with_type("application/epub+zip"),
    );
    feed.entries.push(entry);

    // Generate to XML (generation succeeds)
    let xml = generator
        .generate_feed(&feed)
        .expect("Failed to generate XML");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&xml).expect("Failed to parse XML");

    // Validation should fail on empty ID
    let result = parser.validate_feed(&parsed_feed);
    assert!(result.is_err(), "Validation should fail on empty ID");
}

#[test]
fn test_validation_fails_on_missing_acquisition_link() {
    let generator = Generator::new();

    let mut feed = Feed::new("urn:uuid:invalid-606", "Invalid Feed");

    // Create entry without acquisition link
    let mut entry = Entry::new("urn:uuid:entry-invalid-707", "Entry Without Acq");
    entry
        .links
        .push(Link::new("self", "http://example.com/entry.xml"));
    feed.entries.push(entry);

    // Generate to XML
    let xml = generator
        .generate_feed(&feed)
        .expect("Failed to generate XML");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&xml).expect("Failed to parse XML");

    // Validation should fail on missing acquisition link
    let result = parser.validate_feed(&parsed_feed);
    assert!(
        result.is_err(),
        "Validation should fail on missing acquisition link"
    );
}

#[test]
fn test_nested_indirect_acquisition_validation() {
    let generator = Generator::new();

    let mut feed = Feed::new("urn:uuid:validation-808", "Validation Feed");
    let mut entry = Entry::new("urn:uuid:entry-validation-909", "Validation Entry");

    // Create a nested acquisition with empty child type (invalid)
    let outer_acq = Generator::nested_indirect_acquisition(
        "application/vnd.adobe.adept+xml",
        "", // Empty type - should fail validation
    );

    let mut link = Link::new(
        "http://opds-spec.org/acquisition",
        "http://example.com/book.pdf",
    )
    .with_type("application/pdf");
    link = link.with_indirect_acquisition(outer_acq);
    entry.links.push(link);
    feed.entries.push(entry);

    // Generate to XML
    let xml = generator
        .generate_feed(&feed)
        .expect("Failed to generate XML");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&xml).expect("Failed to parse XML");

    // Validation should fail on empty child type in nested acquisition
    let result = parser.validate_feed(&parsed_feed);
    assert!(
        result.is_err(),
        "Validation should fail on empty indirect acquisition type"
    );
}

#[test]
fn test_round_trip_field_by_field_validation() {
    let generator = Generator::new();

    // Create complex feed
    let mut feed = Feed::new("urn:uuid:complete-555", "Complete Feed");
    feed.rights = Some("Copyright 2024".to_string());
    feed.icon = Some("http://example.com/icon.png".to_string());
    feed.generator = Some("Test Generator".to_string());

    feed.links.push(
        Link::new("self", "http://example.com/feed.xml")
            .with_type("application/atom+xml;profile=opds-catalog;kind=navigation")
            .with_title("Main Feed"),
    );
    feed.links.push(
        Link::new("search", "http://example.com/search")
            .with_type("application/opensearchdescription+xml"),
    );

    // Save the original entry for comparison
    let original_link = Link::new(
        "http://opds-spec.org/acquisition",
        "http://example.com/book.epub",
    )
    .with_type("application/epub+zip")
    .with_hreflang("en")
    .with_title("EPUB Version");

    let mut entry = Entry::new("urn:uuid:entry-complete-666", "Complete Entry");
    entry.published = Some(chrono::Utc::now());
    entry.rights = Some("Some rights reserved".to_string());
    entry.dc_identifier = Some(vec!["urn:isbn:123456".to_string()]);
    entry.dc_title = Some(vec!["Full Title".to_string()]);
    entry.dc_language = Some(vec!["en".to_string()]);

    entry.links.push(original_link.clone());
    feed.entries.push(entry);

    // Generate to XML
    let xml = generator
        .generate_feed(&feed)
        .expect("Failed to generate XML");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&xml).expect("Failed to parse XML");

    // Field-by-field validation
    assert_eq!(parsed_feed.id, feed.id);
    assert_eq!(parsed_feed.title, feed.title);
    assert_eq!(parsed_feed.rights, feed.rights);
    assert_eq!(parsed_feed.icon, feed.icon);
    assert_eq!(parsed_feed.generator, feed.generator);

    assert_eq!(parsed_feed.links.len(), feed.links.len());
    assert_eq!(parsed_feed.links[0].rel, feed.links[0].rel);
    assert_eq!(parsed_feed.links[0].href, feed.links[0].href);
    assert_eq!(parsed_feed.links[0].title, feed.links[0].title);
    assert_eq!(parsed_feed.links[0].type_, feed.links[0].type_);

    assert_eq!(parsed_feed.entries.len(), feed.entries.len());
    let parsed_entry = &parsed_feed.entries[0];
    assert_eq!(parsed_entry.id, "urn:uuid:entry-complete-666");
    assert_eq!(parsed_entry.title, "Complete Entry");
    assert_eq!(
        parsed_entry.rights,
        Some("Some rights reserved".to_string())
    );
    assert_eq!(
        parsed_entry.dc_identifier,
        Some(vec!["urn:isbn:123456".to_string()])
    );
    assert_eq!(parsed_entry.dc_title, Some(vec!["Full Title".to_string()]));
    assert_eq!(parsed_entry.dc_language, Some(vec!["en".to_string()]));

    let parsed_link = &parsed_entry.links[0];
    assert_eq!(parsed_link.rel, "http://opds-spec.org/acquisition");
    assert_eq!(parsed_link.href, "http://example.com/book.epub");
    assert_eq!(parsed_link.hreflang, Some("en".to_string()));
    assert_eq!(parsed_link.title, Some("EPUB Version".to_string()));
    assert_eq!(parsed_link.type_, Some("application/epub+zip".to_string()));

    // Final validation
    parser
        .validate_feed(&parsed_feed)
        .expect("Final validation failed");
}
