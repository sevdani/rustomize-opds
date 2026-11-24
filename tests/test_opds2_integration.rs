//! Integration tests for OPDS 2.0 parser and generator

use rustomize_opds::opds2::{Generator, Link, LinkWithType, Parser};

#[test]
fn test_parse_minimal_opds2_feed() {
    let parser = Parser::new();

    // Read the minimal feed fixture
    let json = std::fs::read_to_string("tests/fixtures/opds2/minimal_feed.json")
        .expect("Failed to read fixture file");

    // Parse the feed
    let feed = parser.parse_feed(&json).expect("Failed to parse feed");

    // Verify feed metadata (now nested)
    assert_eq!(
        feed.metadata
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .unwrap()
            .get_primary(),
        Some("Minimal OPDS 2.0 Feed".to_string())
    );
    assert!(feed.metadata.as_ref().unwrap().identifier.is_some());

    // Verify links
    assert_eq!(feed.feed_links.as_ref().unwrap().len(), 1);
    assert_eq!(
        feed.feed_links.as_ref().unwrap()[0].rel,
        Some(vec!["self".to_string()])
    );
    assert_eq!(
        feed.feed_links.as_ref().unwrap()[0].href,
        Some("http://example.com/opds2/minimal.json".to_string())
    );

    // Verify publications collection exists
    assert!(feed.publications.is_some());
    let publications = feed.publications.as_ref().unwrap();
    assert!(!publications.is_empty());

    // Verify first publication
    let publication = &publications[0];
    assert_eq!(
        publication
            .metadata
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .unwrap()
            .get_primary(),
        Some("Sample Publication".to_string())
    );
    assert!(publication.metadata.as_ref().unwrap().type_.is_some());
}

#[test]
fn test_parse_full_opds2_feed() {
    let parser = Parser::new();

    // Read the full feed fixture
    let json = std::fs::read_to_string("tests/fixtures/opds2/full_feed.json")
        .expect("Failed to read fixture file");

    // Parse the feed
    let feed = parser.parse_feed(&json).expect("Failed to parse feed");

    // Verify feed metadata (now nested)
    assert_eq!(
        feed.metadata
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .unwrap()
            .get_primary(),
        Some("Complete OPDS 2.0 Catalog".to_string())
    );
    assert!(feed.metadata.as_ref().unwrap().subtitle.is_some());
    assert_eq!(feed.metadata.as_ref().unwrap().items_per_page, Some(50));
    assert_eq!(feed.metadata.as_ref().unwrap().current_page, Some(1));
    assert_eq!(feed.metadata.as_ref().unwrap().number_of_items, Some(1000));

    // Verify multiple link types
    let feed_links = feed.feed_links.as_ref().unwrap();
    assert_eq!(feed_links.len(), 3);
    let self_link = feed_links
        .iter()
        .find(|l| {
            l.rel
                .as_ref()
                .map(|rels| rels.contains(&"self".to_string()))
                .unwrap_or(false)
        })
        .expect("Should have self link");
    assert_eq!(
        self_link.href,
        Some("http://example.com/opds2/full.json".to_string())
    );

    // Verify navigation collection
    assert!(feed.navigation.is_some());
    let navigation = feed.navigation.as_ref().unwrap();
    assert!(!navigation.is_empty());

    // Verify publications collection
    assert!(feed.publications.is_some());
    let publications = feed.publications.as_ref().unwrap();
    assert!(publications.len() >= 2);

    // Verify first publication has complex metadata
    let publication = &publications[0];
    assert_eq!(
        publication
            .metadata
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .unwrap()
            .get_primary(),
        Some("The Art of Programming".to_string())
    );
    assert!(publication.metadata.as_ref().unwrap().author.is_some());
    assert!(publication.metadata.as_ref().unwrap().language.is_some());

    // Verify links with properties
    let links = publication.links.as_ref().unwrap();
    let acquisition_link = links
        .iter()
        .find(|l| {
            l.rel
                .as_ref()
                .map(|rels| rels.contains(&"acquisition".to_string()))
                .unwrap_or(false)
        })
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

    // Verify basic properties match (using metadata.title)
    assert_eq!(
        reparsed_feed
            .metadata
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .unwrap()
            .get_primary(),
        feed.metadata
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .unwrap()
            .get_primary()
    );
}

#[test]
fn test_generate_minimal_structures() {
    // Test generating minimal feed
    let feed = Generator::generate_minimal_feed("Test Catalog", "http://example.com/catalog.json");

    assert_eq!(
        feed.metadata
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .unwrap()
            .get_primary(),
        Some("Test Catalog".to_string())
    );
    assert_eq!(feed.feed_links.as_ref().unwrap().len(), 1);
    assert_eq!(
        feed.feed_links.as_ref().unwrap()[0].rel,
        Some(vec!["self".to_string()])
    );
    assert_eq!(
        feed.feed_links.as_ref().unwrap()[0].type_,
        Some("application/opds+json".to_string())
    );

    // Test generating minimal publication
    let publication = Generator::generate_minimal_publication(
        "Test Book",
        "http://example.com/book.json",
        "http://example.com/book.epub",
    );

    assert_eq!(
        publication
            .metadata
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .unwrap()
            .get_primary(),
        Some("Test Book".to_string())
    );
    assert_eq!(publication.links.as_ref().unwrap().len(), 2);

    let download_link = publication
        .links
        .as_ref()
        .unwrap()
        .iter()
        .find(|l| {
            l.rel
                .as_ref()
                .map(|rels| rels.contains(&"download".to_string()))
                .unwrap_or(false)
        })
        .expect("Should have download link");

    assert_eq!(
        download_link.href,
        Some("http://example.com/book.epub".to_string())
    );
}

#[test]
fn test_generate_search_link() {
    let search_link = Generator::generate_search_link("search{?query}");

    assert_eq!(search_link.rel, Some(vec!["search".to_string()]));
    assert_eq!(search_link.href, Some("search{?query}".to_string()));
    assert_eq!(search_link.type_, Some("application/opds+json".to_string()));
    assert_eq!(search_link.templated, Some(true));
}

#[test]
fn test_contributor_with_links_validation() {
    use rustomize_opds::opds2::{Contributor, Link};

    // Valid contributor with links
    let contributor =
        Contributor::new("Test Author").with_links(vec![Link::new("http://example.com/author")
            .with_rel("self")
            .with_type("application/opds+json")]);

    assert!(contributor.validate().is_ok());
}

#[test]
fn test_facet_validation_requires_multiple_links() {
    use rustomize_opds::opds2::Facet;

    // Facet with only one link should fail
    let facet = Facet::new("Test Facet", Some(vec![Link::new("http://example.com")]));

    let result = facet.validate();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("at least two links"));
}

#[test]
fn test_author_with_links() {
    use rustomize_opds::opds2::{Author, Link};

    // Create author with links
    let author =
        Author::new("Test Author").with_links(vec![Link::new("http://example.com/author")
            .with_rel("self")
            .with_type("application/opds+json")]);

    assert_eq!(author.name, Some("Test Author".to_string()));
    assert!(author.links.is_some());
    assert_eq!(author.links.as_ref().unwrap().len(), 1);
}

#[test]
fn test_contributor_with_links() {
    use rustomize_opds::opds2::{Contributor, Link};

    // Create contributor with links
    let contributor = Contributor::new("Test Translator").with_links(vec![Link::new(
        "http://example.com/translator",
    )
    .with_rel("self")
    .with_type("application/opds+json")]);

    assert_eq!(contributor.name, Some("Test Translator".to_string()));
    assert!(contributor.links.is_some());
    assert_eq!(contributor.links.as_ref().unwrap().len(), 1);
}
