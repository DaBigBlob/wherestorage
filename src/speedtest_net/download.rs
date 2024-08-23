use crate::prelude::*;
use url::Url;

pub async fn download_file(_url: Url) -> Result<()> {
    Err(Error::from_str("Unimplemented"))
}
