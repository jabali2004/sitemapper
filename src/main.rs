use clap::{ArgAction, Parser};
use log::{info, warn};

mod extractor;
use env_logger::Env;
use extractor::extract_urls;

mod generator;
use generator::generate_sitemap;

mod validator;
use validator::validated_url;

/// Sitemapper: CLI tool for the creation of sitemaps.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Disable browser mode
    // #[arg(long = "disable-browser-mode", action=ArgAction::SetFalse)]
    // disable_browser_mode: Option<bool>,
    /// Url used for the creation for the sitemap
    url: String,
}

fn main() {
    init();

    let args = Args::parse();
    let target_location = args.url.as_str();

    if !validated_url(&target_location) {
        warn!("A target url is needed!")
    }

    info!("Start extracting urls from page: {}", &target_location);

    let extracted_urls = extract_urls(&target_location);
    match extracted_urls {
        Ok(urls) => generate_sitemap(urls, &target_location),
        Err(_) => warn!("Something weird happened!"),
    }
}

/// Init all dependents
fn init() -> () {
    init_logging();
}

/// Init logging provider
fn init_logging() -> () {
    env_logger::Builder::from_env(Env::default().default_filter_or("info,headless_chrome=warn"))
        .init();
}

#[cfg(test)]
mod tests {}
