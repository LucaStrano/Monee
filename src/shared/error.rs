use core::fmt;


#[derive(Debug)]
/// Represents the possible errors that can occur when fetching system metrics.
pub enum FetchError{
    /// An error indicating that an invalid value was encountered.
    InvalidValue
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::InvalidValue => write!(f, "Invalid value encountered"),
        }
    }
}

impl std::error::Error for FetchError {}