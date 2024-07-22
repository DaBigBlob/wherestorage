
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

pub type Result<T> = core::result::Result<T, Error>;