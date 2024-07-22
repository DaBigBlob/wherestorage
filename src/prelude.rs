
use std::fmt::Display;

pub enum Error {
    WithMessage(String),
    UnderlyingError(String)
}

#[allow(dead_code)]
impl Error {
    pub fn from_err(e: impl Display) -> Self {
        Error::UnderlyingError(e.to_string())
    }

    pub fn from_str(s: &str) -> Self {
        Error::WithMessage(s.to_string())
    }

    pub fn to_string(self) -> String {
        match self {
            Self::UnderlyingError(s) => s,
            Self::WithMessage(s) => s
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;