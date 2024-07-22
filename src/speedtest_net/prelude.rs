
use std::{fmt::Display, io::{stdout, Write}, process};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    UploadResultIDNotu64,
    UploadNoResultIDInResponse,
    UploadJsonParseError(reqwest::Error),
    UploadResponseError(reqwest::Error),
    UploadJsonBuildError(serde_json::Error),
    UploadHttpClientInitError(reqwest::Error),
    UploadPayloadCreationError(String),
    Unimplemented
}

// pub enum Error {
//     Message(String),
//     Underlying(Error)
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