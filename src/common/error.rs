//! OPDS Parsing and Generation Errors

use thiserror::Error;

/// Result type for OPDS operations
pub type Result<T> = std::result::Result<T, OpdsError>;

/// Error types for OPDS parsing and generation
#[derive(Error, Debug)]
pub enum OpdsError {
    /// XML parsing error
    #[error("XML parsing error: {0}")]
    Xml(#[from] serde_xml_rs::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// Validation error for OPDS content
    #[error("Validation error: {0}")]
    Validation(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid media type
    #[error("Invalid media type: {0}")]
    InvalidMediaType(String),

    /// Invalid currency code
    #[error("Invalid currency code: {0}")]
    InvalidCurrency(String),

    /// IO error during file operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Custom error message
    #[error("{0}")]
    Custom(String),
}

/// Validation result type
pub type ValidationResult = Result<()>;

/// Extension trait for validation
pub trait Validate {
    /// Validate the structure and return an error if invalid
    fn validate(&self) -> ValidationResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opds_error_variants() {
        // Xml error - use a simple error variant
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let xml_error = OpdsError::Xml(serde_xml_rs::Error::Io { source: io_error });
        assert!(matches!(xml_error, OpdsError::Xml(_)));

        // Json error - use io variant
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let json_error = OpdsError::Json(serde_json::Error::io(io_error));
        assert!(matches!(json_error, OpdsError::Json(_)));

        // Validation error
        let validation_error = OpdsError::Validation("test".to_string());
        assert!(matches!(validation_error, OpdsError::Validation(_)));

        // Missing field error
        let missing_error = OpdsError::MissingField("test".to_string());
        assert!(matches!(missing_error, OpdsError::MissingField(_)));

        // Invalid media type error
        let media_error = OpdsError::InvalidMediaType("test".to_string());
        assert!(matches!(media_error, OpdsError::InvalidMediaType(_)));

        // Invalid currency error
        let currency_error = OpdsError::InvalidCurrency("test".to_string());
        assert!(matches!(currency_error, OpdsError::InvalidCurrency(_)));

        // IO error
        let io_error = OpdsError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        assert!(matches!(io_error, OpdsError::Io(_)));

        // Custom error
        let custom_error = OpdsError::Custom("test".to_string());
        assert!(matches!(custom_error, OpdsError::Custom(_)));
    }

    #[test]
    fn test_validation_result_type() {
        let result: ValidationResult = Ok(());
        assert!(result.is_ok());

        let result: ValidationResult = Err(OpdsError::Validation("test".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_trait() {
        // Test that types implement the Validate trait
        use crate::CommonMetadata;
        let metadata = CommonMetadata::new("Test");
        let _ = metadata.validate();

        use crate::opds2::Link;
        let link = Link::new("self", "http://example.com");
        let _ = link.validate();
    }
}
