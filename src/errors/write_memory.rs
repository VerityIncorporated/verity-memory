
#[derive(Debug, PartialEq)]
pub enum WriteMemoryError {
    NullPointer,
    InvalidAlignment,
    InvalidAccess,
    FailedToChangeProtection,
    FailedToRestoreProtection
}

impl std::fmt::Display for WriteMemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for WriteMemoryError {}