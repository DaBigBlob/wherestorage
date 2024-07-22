
mod prelude;
mod cli;
mod download;
mod upload;
mod utils;
mod speedtest_net;

use prelude::*;
use clap::Parser;
use cli::{Args, Subcmd};
use download::download;
use upload::upload;

#[tokio::main]
async fn main() -> Result<()> {
    match Args::parse().command {
        Subcmd::Down { url, depth } => download(url, depth).await,
        Subcmd::Up { file, retries } => upload(file, retries).await,
    }
}


