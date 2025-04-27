use std::fmt;

#[derive(Debug, Clone)]
pub enum SolendError {
    TransactionTooLarge,
    ConversionWouldOverflow,
    FailedToParse,
    UnknownError
}

impl fmt::Display for SolendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TransactionTooLarge => write!(f, "Transaction is too large to process!"),
            Self::ConversionWouldOverflow => write!(f, "This attempted conversion would overflow!"),
            Self::FailedToParse => write!(f, "Could not parse the given data"),
            Self::UnknownError => write!(f, "Unknown Error occured.")
        }
    }
}
