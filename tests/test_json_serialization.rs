//! Test JSON serialization field names

use rustomize_opds::opds2::{LanguageMap, LinkProperties, PublicationMetadata};

#[test]
fn test_pagination_field_serialization() {
    let metadata = PublicationMetadata {
        items_per_page: Some(50),
        current_page: Some(1),
        number_of_items: Some(100),
        title: Some(LanguageMap::new("Test")),
        ..PublicationMetadata::default()
    };

    let json = serde_json::to_string_pretty(&metadata).unwrap();
    println!("Metadata JSON:\n{}", json);

    assert!(
        json.contains("itemsPerPage"),
        "Should serialize as itemsPerPage (camelCase)"
    );
    assert!(
        json.contains("currentPage"),
        "Should serialize as currentPage (camelCase)"
    );
    assert!(
        json.contains("numberOfItems"),
        "Should serialize as numberOfItems (camelCase)"
    );
}

#[test]
fn test_link_properties_serialization() {
    let props = LinkProperties {
        number_of_items: Some(10),
        indirect_acquisition: Some(vec![]),
        price: None,
        availability: None,
        holds: None,
        copies: None,
    };

    let json = serde_json::to_string_pretty(&props).unwrap();
    println!("LinkProperties JSON:\n{}", json);

    assert!(
        json.contains("numberOfItems"),
        "Should serialize as numberOfItems (camelCase)"
    );
    assert!(
        json.contains("indirectAcquisition"),
        "Should serialize as indirectAcquisition (camelCase)"
    );
}

#[test]
fn test_roundtrip_with_camelcase() {
    // Test that we can deserialize camelCase and serialize to camelCase
    let json = r#"{"title":"Test","itemsPerPage":50,"currentPage":1}"#;

    let metadata: PublicationMetadata = serde_json::from_str(json).unwrap();
    assert_eq!(metadata.items_per_page, Some(50));
    assert_eq!(metadata.current_page, Some(1));

    // Serialize back
    let output_json = serde_json::to_string(&metadata).unwrap();
    println!("Output JSON: {}", output_json);
    assert!(output_json.contains("itemsPerPage"));
    assert!(output_json.contains("currentPage"));
}
