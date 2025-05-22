use std::fmt;

#[derive(Debug, Clone)]
pub enum KaminoError {
    InvalidObligationType,
    FailedToFetch,
    FailedToParse,
    ConversionWouldOverflow,
    InvalidProgramData,
    UnknownError,
    Invalid
}

impl fmt::Display for KaminoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidObligationType => write!(f, "Invalid obligation type passed"),
            Self::FailedToFetch => write!(f, "Failed to fetch!"),
            Self::FailedToParse => write!(f, "Failed to parse account data"),
            Self::ConversionWouldOverflow => write!(f, "Could not convert number without overflow!"),
            Self::Invalid => write!(f, "Tried to pass invalid data"),
            _ => write!(f, "an Unknown Error occured")
        }
    }
}