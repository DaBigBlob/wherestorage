
use crate::prelude::*;
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue}, Client};

pub fn client() -> Result<Client> {
    Client::builder()
        .default_headers(headermap())
        .deflate(true)
        .gzip(true)
        .brotli(true)
        .zstd(true)
        .build()
        .map_err(|e| Error::from_err(e))        
}

fn headermap() -> HeaderMap {
    let mut h = HeaderMap::new();

    // {
    //     "User-Agent": "Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0",
    //     "Accept": "application/json, text/plain, */*",
    //     "Accept-Language": "en-US,en;q=0.5",
    //     "Content-Type": "application/json;charset=UTF-8",
    //     "Sec-GPC": "1",
    //     "Sec-Fetch-Dest": "empty",
    //     "Sec-Fetch-Mode": "cors",
    //     "Sec-Fetch-Site": "same-origin"
    // }

    h.insert(
        HeaderName::from_static("user-agent"),
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0")
    );
    h.insert(
        HeaderName::from_static("accept"),
        HeaderValue::from_static("application/json, text/plain, */*")
    );
    h.insert(
        HeaderName::from_static("accept-language"),
        HeaderValue::from_static("en-US,en;q=0.5")
    );
    h.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=UTF-8")
    );
    h.insert(
        HeaderName::from_static("sec-gpc"),
        HeaderValue::from_static("1")
    );
    h.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("empty")
    );
    h.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("cors")
    );
    h.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("same-origin")
    );
    h.insert(
        HeaderName::from_static("referer"),
        HeaderValue::from_static("https://www.speedtest.net/")
    );


    h
}