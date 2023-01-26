use clap::{ArgAction, Parser};
use env_logger::Env;
use log::{info, warn};

mod extractor;
use extractor::extract_urls;

mod generator;
use generator::generate_sitemap;

mod validator;
use url::Url;
use validator::validated_url;

/// Sitemapper: CLI tool for the creation of sitemaps.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Disable browser mode
    // #[arg(long = "disable-browser-mode", action=ArgAction::SetFalse)]
    // disable_browser_mode: Option<bool>,

    // Adds a trailing slash to the generated urls
    #[arg(long="trailing-slash", action=ArgAction::SetTrue)]
    trailing_slash: Option<bool>,

    /// Url used for the creation for the sitemap
    url: String,
}

fn main() {
    init();

    let args = Args::parse();
    let url_str = args.url.as_str();

    if !validated_url(&url_str) {
        warn!("A target url is needed!")
    }

    let target_url = Url::parse(url_str).expect("Unable to parse URL!");

    info!("Start extracting urls from page: {}", &url_str);

    let extracted_urls = extract_urls(&target_url);
    match extracted_urls {
        Ok(urls) => generate_sitemap(&target_url, urls, &args),
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
