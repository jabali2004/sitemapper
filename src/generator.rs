use chrono::FixedOffset;
use log::{info, warn};
use sitemap::{structs::UrlEntry, writer::SiteMapWriter};
use std::{fs::File, io::LineWriter};
use url::Url;

use crate::Args;

static DEFAULT_PATH: &str = "./sitemap.xml";

/// Generate a site map for given urls
pub fn generate_sitemap(target_url: &Url, url_list: Vec<String>, args: &Args) {
    let file_path = DEFAULT_PATH;
    let file =
        File::create(file_path).expect(&format!("Could not open and or create file {}", file_path));

    let sitemap_writer = SiteMapWriter::new(LineWriter::new(file));
    let url_writer = sitemap_writer.start_urlset();

    let mut validated_urls: Vec<String> = url_list
        .iter()
        .map(|url| format_url(&target_url, url.as_str(), &args))
        .collect();

    validated_urls.sort();
    validated_urls.dedup();

    match url_writer {
        Ok(mut writer) => {
            for validated_url in validated_urls {
                info!("Add sitemap entry for: {}", validated_url);
                let url_entry = UrlEntry::builder()
                    .loc(&validated_url)
                    .lastmod(build_lastmod());

                match writer.url(url_entry) {
                    Ok(_) => info!("Writing: {}", validated_url),
                    Err(err) => warn!("Error while writing url: {}", err.to_string()),
                }
            }

            match writer.end() {
                Ok(_) => info!("Finished creating sitemap: {}", file_path),
                Err(err) => warn!("Unable to end url writer: {}", err.to_string()),
            }
        }
        Err(err) => warn!("Unable to establish url writer: {}", err.to_string()),
    }
}

/// Build lastmod value for sitemap entry
fn build_lastmod() -> chrono::DateTime<FixedOffset> {
    return chrono::offset::Utc::now().with_timezone(&FixedOffset::west_opt(0).unwrap());
}

/// Format url to be a valid sitemap url
fn format_url(target_url: &Url, url: &str, args: &Args) -> String {
    let path = match url.starts_with("/") {
        true => split_path(url),
        false => {
            let parsed_url = Url::parse(url).expect("Unable to parse URL!");
            split_path(parsed_url.path())
        }
    };

    let trailing_slash = match args.trailing_slash.unwrap() && !path.ends_with("/") {
        true => "/",
        false => "",
    };

    return format!(
        "{}://{}{}{}",
        target_url.scheme(),
        target_url.host_str().unwrap(),
        path,
        trailing_slash,
    );
}

/// Split path
fn split_path(url: &str) -> String {
    let coll = url.split("#").collect::<Vec<&str>>();

    match coll.first() {
        Some(element) => String::from(*element),
        None => {
            warn!("Unable to get first element of vec from URL: {}", url);
            String::new()
        }
    }
}
