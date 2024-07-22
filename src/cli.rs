
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
    /// Upload a file
    Up {
        /// The file you want to upload
        #[arg()]
        file: PathBuf
    },

    /// Download a file
    Down {
        /// The speedtest.net url
        #[arg()]
        url: Url
    }
}
