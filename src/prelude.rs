
use std::{fmt::Display, io::{stdout, Write}, process};


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

pub fn resolve<T: Display>(r: Result<T>) {
    let mut stdo = stdout().lock();
    match r {
        Ok(t) => {
            match stdo.write_all(format!("{}",t).as_bytes()) {
                Ok(_) => (),
                Err(_) => panic!("Cannot write to stdout.")
            }
            process::exit(1);
        },

        Err(_) => {
            match stdo.write_all(format!("").as_bytes()) {
                Ok(_) => (),
                Err(_) => panic!("Cannot write to stdout.")
            }
            process::exit(1);
        }
    }
}