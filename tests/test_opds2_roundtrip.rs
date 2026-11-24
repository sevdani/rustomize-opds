//! Integration tests for OPDS 2.0 generator and parser round-trip validation

use rustomize_opds::opds2::{
    AcquisitionObject, Availability, AvailabilityState, Copies, Feed, Generator, Holds, Link,
    LinkProperties, LinkWithType, Parser, Price, Publication, PublicationMetadata,
};
use rustomize_opds::opds2::{Author, SeriesInfo};
use rustomize_opds::opds2::{LinkWithProperties, PublicationWithLink};

#[test]
fn test_generate_parse_minimal_feed() {
    let generator = Generator::new();

    // Generate a minimal feed
    let mut original_feed =
        Generator::generate_minimal_feed("Generated OPDS 2 Feed", "http://example.com/feed.json");

    // Add publications collection (required for OPDS 2.0)
    let publication = Generator::generate_minimal_publication(
        "Sample Book",
        "http://example.com/book.json",
        "http://example.com/book.epub",
    );
    original_feed = original_feed.with_publications(vec![publication]);

    // Generate to JSON (with auto-validation)
    let json = generator
        .generate_and_validate_feed(&original_feed)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&json).expect("Failed to parse JSON");

    // Validate basic fields
    assert_eq!(parsed_feed.title, original_feed.title);

    // Validate links
    assert_eq!(parsed_feed.feed_links.len(), original_feed.feed_links.len());
    assert_eq!(
        parsed_feed.feed_links[0].rel,
        original_feed.feed_links[0].rel
    );
    assert_eq!(
        parsed_feed.feed_links[0].href,
        original_feed.feed_links[0].href
    );

    // Validate publications
    assert!(parsed_feed.publications.is_some());
    assert_eq!(parsed_feed.publications.as_ref().unwrap().len(), 1);

    // Validate parsed feed
    parser
        .validate_feed(&parsed_feed)
        .expect("Parsed feed validation failed");
}

#[test]
fn test_generate_parse_feed_with_publications() {
    let generator = Generator::new();

    // Create feed with publications
    let mut feed = Feed::new(
        "Feed with Publications",
        vec![Link::new("self", "http://example.com/feed.json").with_type("application/opds+json")],
    );

    // Add publications
    let pub1 = Generator::generate_minimal_publication(
        "Book One",
        "http://example.com/book1.json",
        "http://example.com/book1.epub",
    );
    let pub2 = Generator::generate_minimal_publication(
        "Book Two",
        "http://example.com/book2.json",
        "http://example.com/book2.pdf",
    );

    feed = feed.with_publications(vec![pub1, pub2]);

    // Generate to JSON (with auto-validation)
    let json = generator
        .generate_and_validate_feed(&feed)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&json).expect("Failed to parse JSON");

    // Validate publications count
    assert!(parsed_feed.publications.is_some());
    let publications = parsed_feed.publications.as_ref().unwrap();
    assert_eq!(publications.len(), 2);

    // Validate first publication
    let pub1_parsed = &publications[0];
    assert_eq!(pub1_parsed.metadata.title, Some("Book One".to_string()));
    assert_eq!(pub1_parsed.links.as_ref().unwrap().len(), 2);

    // Validate parsed feed
    parser
        .validate_feed(&parsed_feed)
        .expect("Parsed feed validation failed");
}

#[test]
fn test_generate_parse_feed_with_navigation() {
    let generator = Generator::new();

    let mut feed = Feed::new(
        "Feed with Navigation",
        vec![Link::new("self", "http://example.com/feed.json").with_type("application/opds+json")],
    );

    // Add navigation links
    feed = feed.with_navigation(vec![
        Link::new("navigation", "http://example.com/fiction")
            .with_title("Fiction")
            .with_type("application/opds-publication+json"),
        Link::new("navigation", "http://example.com/non-fiction")
            .with_title("Non-Fiction")
            .with_type("application/opds-publication+json"),
    ]);

    // Generate to JSON (with auto-validation)
    let json = generator
        .generate_and_validate_feed(&feed)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&json).expect("Failed to parse JSON");

    // Validate navigation
    assert!(parsed_feed.navigation.is_some());
    let navigation = parsed_feed.navigation.as_ref().unwrap();
    assert_eq!(navigation.len(), 2);
    assert_eq!(navigation[0].title, Some("Fiction".to_string()));
    assert_eq!(navigation[1].title, Some("Non-Fiction".to_string()));

    // Validate parsed feed
    parser
        .validate_feed(&parsed_feed)
        .expect("Parsed feed validation failed");
}

#[test]
fn test_generate_parse_feed_with_pagination() {
    let generator = Generator::new();

    let mut feed = Feed::new(
        "Paginated Feed",
        vec![Link::new("self", "http://example.com/feed.json").with_type("application/opds+json")],
    );
    feed.items_per_page = Some(50);
    feed.current_page = Some(3);
    feed.number_of_items = Some(500);

    // Add publications collection (required for OPDS 2.0)
    let publication = Generator::generate_minimal_publication(
        "Sample Book",
        "http://example.com/book.json",
        "http://example.com/book.epub",
    );
    feed = feed.with_publications(vec![publication]);

    // Generate to JSON (with auto-validation)
    let json = generator
        .generate_and_validate_feed(&feed)
        .expect("Failed to generate JSON");

    // Verify pagination field names in JSON
    assert!(json.contains("itemsPerPage"));
    assert!(json.contains("currentPage"));
    assert!(json.contains("numberOfItems"));

    // Parse back
    let parser = Parser::new();
    let parsed_feed = parser.parse_feed(&json).expect("Failed to parse JSON");

    // Validate pagination fields
    assert_eq!(parsed_feed.items_per_page, Some(50));
    assert_eq!(parsed_feed.current_page, Some(3));
    assert_eq!(parsed_feed.number_of_items, Some(500));

    // Validate parsed feed
    parser
        .validate_feed(&parsed_feed)
        .expect("Parsed feed validation failed");
}

#[test]
fn test_generate_parse_publication_with_link_properties() {
    let generator = Generator::new();

    // Create publication with link properties
    let metadata = PublicationMetadata::new("Publication with Properties");
    let mut pub_ = Publication::new(
        metadata,
        Some(vec![Link::new("self", "http://example.com/pub.json")
            .with_type("application/opds-publication+json")]),
    );

    // Add acquisition link with properties
    let acquisition_link = Link::new("acquisition", "http://example.com/book.epub")
        .with_type("application/epub+zip")
        .with_title("Download EPUB");

    // Add price
    let price = Price {
        currency: "USD".to_string(),
        value: 19.99,
    };

    // Add indirect acquisition
    let indirect = AcquisitionObject {
        r#type: "application/epub+zip".to_string(),
        child: None,
    };

    let properties = LinkProperties {
        price: Some(price),
        indirect_acquisition: Some(vec![indirect]),
        number_of_items: Some(10),
        ..LinkProperties::default()
    };

    let acquisition_link = acquisition_link.with_properties(properties);
    pub_ = pub_.with_link(acquisition_link);

    // Generate to JSON (with auto-validation)
    let json = generator
        .generate_and_validate_publication(&pub_)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_pub = parser
        .parse_publication(&json)
        .expect("Failed to parse publication");

    // Validate link properties
    let parsed_link = &parsed_pub.links.as_ref().unwrap()[1]; // acquisition link
    assert!(parsed_link.properties.is_some());

    let props = parsed_link.properties.as_ref().unwrap();
    assert!(props.price.is_some());
    assert_eq!(props.price.as_ref().unwrap().currency, "USD");
    assert_eq!(props.price.as_ref().unwrap().value, 19.99);

    assert!(props.indirect_acquisition.is_some());
    assert_eq!(props.indirect_acquisition.as_ref().unwrap().len(), 1);

    assert_eq!(props.number_of_items, Some(10));

    // Validate parsed publication
    parser
        .validate_publication(&parsed_pub)
        .expect("Parsed publication validation failed");
}

#[test]
fn test_generate_parse_publication_with_availability() {
    let generator = Generator::new();

    let metadata = PublicationMetadata::new("Publication with Availability");
    let mut pub_ = Publication::new(
        metadata,
        Some(vec![Link::new("self", "http://example.com/pub.json")
            .with_type("application/opds-publication+json")]),
    );

    // Add acquisition link with availability
    let properties = LinkProperties {
        availability: Some(Availability {
            state: AvailabilityState::Available,
            since: Some("2024-01-01T00:00:00Z".to_string()),
            until: Some("2024-12-31T23:59:59Z".to_string()),
        }),
        ..LinkProperties::default()
    };

    let acquisition_link = Link::new("acquisition", "http://example.com/book.epub")
        .with_type("application/epub+zip")
        .with_properties(properties);

    pub_ = pub_.with_link(acquisition_link);

    // Generate to JSON (with auto-validation)
    let json = generator
        .generate_and_validate_publication(&pub_)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_pub = parser
        .parse_publication(&json)
        .expect("Failed to parse publication");

    // Validate availability
    let parsed_link = &parsed_pub.links.as_ref().unwrap()[1];
    assert!(parsed_link.properties.is_some());

    let availability = parsed_link
        .properties
        .as_ref()
        .unwrap()
        .availability
        .as_ref()
        .unwrap();
    assert!(matches!(availability.state, AvailabilityState::Available));
    assert_eq!(availability.since, Some("2024-01-01T00:00:00Z".to_string()));
    assert_eq!(availability.until, Some("2024-12-31T23:59:59Z".to_string()));

    // Validate parsed publication
    parser
        .validate_publication(&parsed_pub)
        .expect("Parsed publication validation failed");
}

#[test]
fn test_generate_parse_publication_with_holds_and_copies() {
    let generator = Generator::new();

    let metadata = PublicationMetadata::new("Publication with Library Features");
    let mut pub_ = Publication::new(
        metadata,
        Some(vec![Link::new("self", "http://example.com/pub.json")
            .with_type("application/opds-publication+json")]),
    );

    // Add acquisition link with holds and copies
    let properties = LinkProperties {
        holds: Some(Holds {
            total: 10,
            position: 3,
        }),
        copies: Some(Copies {
            total: 5,
            available: 2,
        }),
        ..LinkProperties::default()
    };

    let acquisition_link = Link::new("acquisition", "http://example.com/book.epub")
        .with_type("application/epub+zip")
        .with_properties(properties);

    pub_ = pub_.with_link(acquisition_link);

    // Generate to JSON (with auto-validation)
    let json = generator
        .generate_and_validate_publication(&pub_)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_pub = parser
        .parse_publication(&json)
        .expect("Failed to parse publication");

    // Validate holds
    let parsed_link = &parsed_pub.links.as_ref().unwrap()[1];
    let holds = parsed_link
        .properties
        .as_ref()
        .unwrap()
        .holds
        .as_ref()
        .unwrap();
    assert_eq!(holds.total, 10);
    assert_eq!(holds.position, 3);

    // Validate copies
    let copies = parsed_link
        .properties
        .as_ref()
        .unwrap()
        .copies
        .as_ref()
        .unwrap();
    assert_eq!(copies.total, 5);
    assert_eq!(copies.available, 2);

    // Validate parsed publication
    parser
        .validate_publication(&parsed_pub)
        .expect("Parsed publication validation failed");
}

#[test]
fn test_validation_fails_on_missing_title() {
    let generator = Generator::new();

    // Create publication with missing title
    let mut pub_ = Publication::new(
        PublicationMetadata::default(),
        Some(vec![Link::new("self", "http://example.com/pub.json")
            .with_type("application/opds-publication+json")]),
    );

    // Add acquisition link
    pub_ = pub_.with_link(Link::new("acquisition", "http://example.com/book.epub"));

    // Generate to JSON (with validate=false to generate invalid data)
    let json = generator
        .generate_publication(&pub_)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_pub = parser
        .parse_publication(&json)
        .expect("Failed to parse publication");

    // Validation should fail on missing title
    let result = parser.validate_publication(&parsed_pub);
    assert!(result.is_err(), "Validation should fail on missing title");
}

#[test]
fn test_validation_fails_on_missing_acquisition_link() {
    let generator = Generator::new();

    let metadata = PublicationMetadata::new("Publication No Acquisition");
    let pub_ = Publication::new(
        metadata,
        Some(vec![Link::new("self", "http://example.com/pub.json")
            .with_type("application/opds-publication+json")]),
    );

    // Generate to JSON (with validate=false to generate invalid data)
    let json = generator
        .generate_publication(&pub_)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_pub = parser
        .parse_publication(&json)
        .expect("Failed to parse publication");

    // Validation should fail on missing acquisition link
    let result = parser.validate_publication(&parsed_pub);
    assert!(
        result.is_err(),
        "Validation should fail on missing acquisition link"
    );
}

#[test]
fn test_validation_fails_on_negative_price() {
    let generator = Generator::new();

    let metadata = PublicationMetadata::new("Publication Negative Price");
    let mut pub_ = Publication::new(
        metadata,
        Some(vec![Link::new("self", "http://example.com/pub.json")
            .with_type("application/opds-publication+json")]),
    );

    let properties = LinkProperties {
        price: Some(Price {
            currency: "USD".to_string(),
            value: -5.00,
        }),
        ..LinkProperties::default()
    };

    let acquisition_link = Link::new("acquisition", "http://example.com/book.epub")
        .with_type("application/epub+zip")
        .with_properties(properties);

    pub_ = pub_.with_link(acquisition_link);

    // Generate to JSON (with validate=false to generate invalid data)
    let json = generator
        .generate_publication(&pub_)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_pub = parser
        .parse_publication(&json)
        .expect("Failed to parse publication");

    // Validation should fail on negative price
    let result = parser.validate_publication(&parsed_pub);
    assert!(result.is_err(), "Validation should fail on negative price");
}

#[test]
fn test_nested_indirect_acquisition_round_trip() {
    let generator = Generator::new();

    let metadata = PublicationMetadata::new("Publication Nested Acq");
    let mut pub_ = Publication::new(
        metadata,
        Some(vec![Link::new("self", "http://example.com/pub.json")
            .with_type("application/opds-publication+json")]),
    );

    // Add nested indirect acquisition (Adobe DRM chain)
    let inner_acq = AcquisitionObject {
        r#type: "application/pdf".to_string(),
        child: None,
    };
    let outer_acq = AcquisitionObject {
        r#type: "application/vnd.adobe.adept+xml".to_string(),
        child: Some(vec![inner_acq]),
    };

    let properties = LinkProperties {
        indirect_acquisition: Some(vec![outer_acq]),
        ..LinkProperties::default()
    };

    let acquisition_link = Link::new("acquisition", "http://example.com/book.pdf")
        .with_type("application/pdf")
        .with_properties(properties);

    pub_ = pub_.with_link(acquisition_link);

    // Generate to JSON (with auto-validation)
    let json = generator
        .generate_and_validate_publication(&pub_)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_pub = parser
        .parse_publication(&json)
        .expect("Failed to parse publication");

    // Validate nested indirect acquisition
    let links = parsed_pub.links.as_ref().unwrap();
    let parsed_link = &links[1];
    let acquisitions = parsed_link
        .properties
        .as_ref()
        .unwrap()
        .indirect_acquisition
        .as_ref()
        .unwrap();
    assert_eq!(acquisitions.len(), 1);

    let outer = &acquisitions[0];
    assert_eq!(outer.r#type, "application/vnd.adobe.adept+xml");
    assert!(outer.child.is_some());

    let inner = outer.child.as_ref().unwrap().first().unwrap();
    assert_eq!(inner.r#type, "application/pdf");

    // Validate parsed publication
    parser
        .validate_publication(&parsed_pub)
        .expect("Parsed publication validation failed");
}

#[test]
fn test_round_trip_with_all_fields() {
    let generator = Generator::new();

    // Create comprehensive publication
    let mut metadata = PublicationMetadata::new("Comprehensive Publication");
    metadata.identifier = Some("urn:isbn:978-1234567890".to_string());
    metadata.subtitle = Some("A Complete Test Case".to_string());
    metadata.modified = Some(chrono::Utc::now());
    metadata.description = Some("This publication tests all fields".to_string());
    metadata.items_per_page = Some(10);
    metadata.current_page = Some(1);
    metadata.number_of_items = Some(100);
    metadata.type_ = Some("http://schema.org/Book".to_string());
    metadata.author = Some(Author {
        name: "Test Author".to_string(),
        identifier: Some("urn:author:1".to_string()),
        sort_as: Some("Author, Test".to_string()),
        links: None,
    });
    metadata.translator = Some("Translator Name".to_string());
    metadata.editor = Some("Editor Name".to_string());
    metadata.illustrator = Some("Illustrator Name".to_string());
    metadata.narrator = Some("Narrator Name".to_string());
    metadata.publisher = Some("Publisher Name".to_string());
    metadata.language = Some("en".to_string());
    metadata.published = Some("2024-01-15".to_string());
    metadata.series = Some(SeriesInfo {
        name: "Series Name".to_string(),
        position: Some(5),
    });
    metadata.collection = Some("Collection Name".to_string());

    let mut pub_ = Publication::new(
        metadata,
        Some(vec![Link::new("self", "http://example.com/pub.json")
            .with_type("application/opds-publication+json")
            .with_title("Self Link")]),
    );

    // Add multiple links with various properties
    pub_ = pub_.with_link(
        Link::new("alternate", "http://example.com/book.html")
            .with_type("text/html")
            .with_title("HTML Version")
            .with_height(100)
            .with_width(200),
    );

    pub_ = pub_.with_link(
        Link::new("acquisition", "http://example.com/book.epub")
            .with_type("application/epub+zip")
            .with_properties(LinkProperties {
                price: Some(Price {
                    currency: "USD".to_string(),
                    value: 19.99,
                }),
                availability: Some(Availability {
                    state: AvailabilityState::Available,
                    since: Some("2024-01-01T00:00:00Z".to_string()),
                    until: None,
                }),
                ..LinkProperties::default()
            }),
    );

    // Generate to JSON (with auto-validation)
    let json = generator
        .generate_and_validate_publication(&pub_)
        .expect("Failed to generate JSON");

    // Parse back
    let parser = Parser::new();
    let parsed_pub = parser
        .parse_publication(&json)
        .expect("Failed to parse publication");

    // Validate metadata
    assert_eq!(
        parsed_pub.metadata.title,
        Some("Comprehensive Publication".to_string())
    );
    assert_eq!(
        parsed_pub.metadata.subtitle,
        Some("A Complete Test Case".to_string())
    );
    assert_eq!(
        parsed_pub.metadata.identifier,
        Some("urn:isbn:978-1234567890".to_string())
    );
    assert_eq!(
        parsed_pub.metadata.author.as_ref().unwrap().name,
        "Test Author"
    );

    // Validate links
    let links = parsed_pub.links.as_ref().unwrap();
    assert_eq!(links.len(), 3);

    // Validate parsed publication
    parser
        .validate_publication(&parsed_pub)
        .expect("Parsed publication validation failed");
}
