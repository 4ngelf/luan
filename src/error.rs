//! Error type for this program
use std::{error::Error, fmt, io, result::Result as StdResult};

pub type Result<T> = StdResult<T, LuanError>;

// Error for this program
pub struct LuanError(lexopt::Error);

impl LuanError {
    pub fn custom<E: Error>(error: E) -> Self {
        let msg = error.to_string().into();
        Self(lexopt::Error::Custom(msg))
    }
}

impl fmt::Debug for LuanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for LuanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<lexopt::Error> for LuanError {
    fn from(value: lexopt::Error) -> Self {
        Self(value)
    }
}

impl From<io::Error> for LuanError {
    fn from(value: io::Error) -> Self {
        Self::custom(value)
    }
}

impl From<&str> for LuanError {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<String> for LuanError {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}
