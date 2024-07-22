
use std::path::PathBuf;
use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(help_template = "{bin} v{version}\n{about}\nAuthor : {author}\n\nUSAGE:\n    {usage}\n\n{all-args}\n")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Action to take
    #[command(subcommand)]
    pub command: Subcmd
}

#[derive(Parser, Debug)]
pub enum Subcmd {
    /// Upload a file to speedtest.net
    Up {
        /// The file you want to upload
        #[arg()]
        file: PathBuf,

        /// Retries before giving up
        #[arg(default_value_t = 60)]
        retries: u64,
    },

    /// Download a file from speedtest.net
    Down {
        /// The speedtest.net url
        #[arg()]
        url: Url,

        /// The depth of the final file
        #[arg()]
        depth: u64
    }
}
