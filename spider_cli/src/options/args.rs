use crate::options::sub_command::Commands;
use clap::Parser;

/// program to crawl a website and gather valid web urls.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Build main sub commands
    #[clap(subcommand)]
    pub command: Option<Commands>,
    /// Domain to crawl
    #[clap(short, long)]
    pub domain: String,
    /// Respect robots.txt file
    #[clap(short, long)]
    pub respect_robots_txt: bool,
    /// Allow sub-domain crawling.
    #[clap(short, long)]
    pub subdomains: bool,
    /// Allow all tlds for domain.
    #[clap(short, long)]
    pub tld: bool,
    /// Print page visited on standard output
    #[clap(short, long)]
    pub verbose: bool,
    /// Polite crawling delay in milli seconds
    #[clap(short = 'D', long)]
    pub delay: Option<u64>,
    /// Comma seperated string list of pages to not crawl or regex with feature enabled
    #[clap(short, long)]
    pub blacklist_url: Option<String>,
    /// User-Agent
    #[clap(short, long)]
    pub user_agent: Option<String>,
}
