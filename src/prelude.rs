use std::fmt::Display;

pub struct Error(String);

#[allow(dead_code)]
impl Error {
    pub fn from_err(e: impl Display) -> Self {
        Self(format!("Underlying Error: {}", e))
    }

    pub fn from_str(str: &str) -> Self {
        Self(str.to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

pub type Result<T> = core::result::Result<T, Error>;
