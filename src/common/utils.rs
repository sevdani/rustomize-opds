//! OPDS Common Utilities

use std::collections::HashMap;

/// Validate ISO 4217 currency codes
pub fn is_valid_currency(code: &str) -> bool {
    const VALID_CURRENCIES: &[&str] = &[
        "AED", "AFN", "ALL", "AMD", "ANG", "AOA", "ARS", "AUD", "AWG", "AZN", "BAM", "BBD", "BDT",
        "BGN", "BHD", "BIF", "BMD", "BND", "BOB", "BOV", "BRL", "BSD", "BTN", "BWP", "BYN", "BZD",
        "CAD", "CDF", "CHE", "CHF", "CHW", "CLF", "CLP", "CNY", "COP", "COU", "CRC", "CUC", "CUP",
        "CVE", "CZK", "DJF", "DKK", "DOP", "DZD", "EGP", "ERN", "ETB", "EUR", "FJD", "FKP", "GBP",
        "GEL", "GHS", "GIP", "GMD", "GNF", "GTQ", "GYD", "HKD", "HNL", "HRK", "HTG", "HUF", "IDR",
        "ILS", "INR", "IQD", "IRR", "ISK", "JMD", "JOD", "JPY", "KES", "KGS", "KHR", "KMF", "KPW",
        "KRW", "KWD", "KYD", "KZT", "LAK", "LBP", "LKR", "LRD", "LSL", "LYD", "MAD", "MDL", "MGA",
        "MKD", "MMK", "MNT", "MOP", "MRU", "MUR", "MVR", "MWK", "MXN", "MXV", "MYR", "MZN", "NAD",
        "NGN", "NIO", "NOK", "NPR", "NZD", "OMR", "PAB", "PEN", "PGK", "PHP", "PKR", "PLN", "PYG",
        "QAR", "RON", "RSD", "RUB", "RWF", "SAR", "SBD", "SCR", "SDG", "SEK", "SGD", "SHP", "SLL",
        "SOS", "SRD", "SSP", "STN", "SVC", "SYP", "SZL", "THB", "TJS", "TMT", "TND", "TOP", "TRY",
        "TTD", "TWD", "TZS", "UAH", "UGX", "USD", "USN", "UYI", "UYU", "UZS", "VEF", "VES", "VND",
        "VUV", "WST", "XAF", "XAG", "XAU", "XBA", "XBB", "XBC", "XBD", "XCD", "XDR", "XOF", "XPD",
        "XPF", "XPT", "XSU", "XTS", "XUA", "XXX", "YER", "ZAR", "ZMW", "ZWL",
    ];
    VALID_CURRENCIES.contains(&code)
}

/// Validate a URL/URI
pub fn is_valid_uri(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://") || s.starts_with("urn:")
}

/// Check if media type is a valid image format
pub fn is_valid_image_format(media_type: &str) -> bool {
    matches!(
        media_type,
        "image/jpeg" | "image/png" | "image/gif" | "image/webp" | "image/avif" | "image/jxl"
    )
}

/// Parse a date string in RFC 3339 format
pub fn parse_date(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&chrono::Utc))
}

/// Format a date in RFC 3339 format
pub fn format_date(dt: &chrono::DateTime<chrono::Utc>) -> String {
    dt.to_rfc3339()
}

/// Merge two hash maps
pub fn merge_maps<K, V: Clone>(a: &HashMap<K, V>, b: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    let mut result = a.clone();
    for (k, v) in b {
        result.insert(k.clone(), v.clone());
    }
    result
}

/// Check if a string is empty or contains only whitespace
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_currency() {
        // Valid currencies
        assert!(is_valid_currency("USD"));
        assert!(is_valid_currency("EUR"));
        assert!(is_valid_currency("GBP"));
        assert!(is_valid_currency("JPY"));
        assert!(is_valid_currency("CAD"));

        // Invalid currencies
        assert!(!is_valid_currency("xyz"));
        assert!(!is_valid_currency(""));
        assert!(!is_valid_currency("US"));
        assert!(!is_valid_currency("INVALID"));
    }

    #[test]
    fn test_is_valid_uri() {
        // Valid URIs
        assert!(is_valid_uri("http://example.com"));
        assert!(is_valid_uri("https://example.com/path"));
        assert!(is_valid_uri("urn:uuid:12345"));

        // Invalid URIs
        assert!(!is_valid_uri("ftp://example.com"));
        assert!(!is_valid_uri("example.com"));
        assert!(!is_valid_uri(""));
    }

    #[test]
    fn test_is_valid_image_format() {
        // Valid image formats
        assert!(is_valid_image_format("image/jpeg"));
        assert!(is_valid_image_format("image/png"));
        assert!(is_valid_image_format("image/gif"));
        assert!(is_valid_image_format("image/webp"));
        assert!(is_valid_image_format("image/avif"));
        assert!(is_valid_image_format("image/jxl"));

        // Invalid formats
        assert!(!is_valid_image_format("image/bmp"));
        assert!(!is_valid_image_format("image/tiff"));
        assert!(!is_valid_image_format("application/pdf"));
        assert!(!is_valid_image_format(""));
    }

    #[test]
    fn test_parse_date() {
        use chrono::Datelike;
        let date_str = "2024-01-15T10:30:00Z";
        let result = parse_date(date_str);
        assert!(result.is_some());

        let dt = result.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 15);
    }

    #[test]
    fn test_parse_date_invalid() {
        let invalid_date = "not-a-date";
        assert!(parse_date(invalid_date).is_none());
    }

    #[test]
    fn test_format_date() {
        use chrono::TimeZone;
        let dt = chrono::Utc
            .timestamp_opt(0, 0)
            .unwrap()
            .with_timezone(&chrono::Utc);
        let formatted = format_date(&dt);
        assert_eq!(formatted, "1970-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_merge_maps() {
        let mut a = HashMap::new();
        a.insert("key1", "value1");
        a.insert("key2", "value2");

        let mut b = HashMap::new();
        b.insert("key2", "value2_modified");
        b.insert("key3", "value3");

        let merged = merge_maps(&a, &b);

        assert_eq!(merged.get("key1"), Some(&"value1"));
        assert_eq!(merged.get("key2"), Some(&"value2_modified"));
        assert_eq!(merged.get("key3"), Some(&"value3"));
    }

    #[test]
    fn test_is_blank() {
        assert!(is_blank(""));
        assert!(is_blank("   "));
        assert!(is_blank("\t\n"));

        assert!(!is_blank("not blank"));
        assert!(!is_blank(" a "));
    }
}
