
mod prelude;
mod cli;
mod speedtest_net;

use clap::Parser;
use cli::{Args, Subcmd};
use speedtest_net::{download_file, upload_file};

#[tokio::main]
async fn main() {
    
    match Args::parse().command {
        Subcmd::Down { url } => download_file(url).await,
        Subcmd::Up { file } => upload_file(file).await,
    }.unwrap_or_else(|e| {
        eprintln!("Error: {}",e.into_string());
        std::process::exit(1);
    })
}


