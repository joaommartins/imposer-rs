use thiserror::Error;

/// Errors that can occur during booklet generation
#[derive(Error, Debug)]
pub enum BookletError {
    /// Error parsing the input PDF
    #[error("Failed to parse PDF: {0}")]
    ParseError(String),

    /// Error generating the output PDF
    #[error("Failed to generate booklet: {0}")]
    GenerationError(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Result type for booklet operations
pub type Result<T> = std::result::Result<T, BookletError>;
