
use crate::prelude::*;
use url::Url;

pub async fn download_file(_url: Url, _depth: u64) -> Result<()> {
    Err(Error::from_str("Unimplemented"))
}