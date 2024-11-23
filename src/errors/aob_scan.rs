
#[derive(Debug, PartialEq)]
pub enum AobScanError {
    PatternNotFound,
    InvalidPattern,
}

impl std::fmt::Display for AobScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AobScanError {}