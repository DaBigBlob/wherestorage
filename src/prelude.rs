
// use std::{fmt::Display, io::{stdout, Write}, process, rc::Rc};

// #[allow(dead_code)]
// #[derive(Debug)]
// pub enum Error {
//     UploadResultIDNotu64,
//     UploadNoResultIDInResponse,
//     UploadJsonParseError(reqwest::Error),
//     UploadResponseError(reqwest::Error),
//     UploadJsonBuildError(serde_json::Error),
//     UploadHttpClientInitError(reqwest::Error),
//     UploadPayloadCreationError(String),
//     Unimplemented
// }

use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    WithMessage(String),
    UnderlyingError(String)
}

impl Error {
    pub fn from_err(e: impl Display) -> Self {
        Error::UnderlyingError(e.to_string())
    }

    pub fn from_str(s: &str) -> Self {
        Error::WithMessage(s.to_string())
    }
}

// impl _Error {
//     fn from_err(e: std::error::Error)-> Self {
//         Self::UnderlyingError(Rc::new(e))
//     }
// }

pub type Result<T> = core::result::Result<T, Error>;

// pub fn resolve_final<T: Display>(r: Result<T>) {
//     let mut stdo = stdout().lock();
//     match r {
//         Ok(t) => {
//             match stdo.write_all(format!("{}",t).as_bytes()) {
//                 Ok(_) => (),
//                 Err(_) => panic!("Cannot write to stdout.")
//             }
//             process::exit(1);
//         },

//         Err(_) => {
//             match stdo.write_all(format!("").as_bytes()) {
//                 Ok(_) => (),
//                 Err(_) => panic!("Cannot write to stdout.")
//             }
//             process::exit(1);
//         }
//     }
// }