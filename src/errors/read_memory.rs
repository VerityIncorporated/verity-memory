
#[derive(Debug, PartialEq)]
pub enum ReadMemoryError {
    NullPointer,
    InvalidAlignment,
    FailedToChangeProtection,
    FailedToRestoreProtection,
    InvalidAccess
}

impl std::fmt::Display for ReadMemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ReadMemoryError {}