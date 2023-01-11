use chrono::FixedOffset;
use log::{info, warn};
use sitemap::{structs::UrlEntry, writer::SiteMapWriter};
use std::{fs::File, io::LineWriter};

static DEFAULT_PATH: &str = "./sitemap.xml";

/// Generate a site map for given urls
pub fn generate_sitemap(url_list: Vec<String>, target_url: &str) {
    let file_path = DEFAULT_PATH;
    let file =
        File::create(file_path).expect(&format!("Could not open and or create file {}", file_path));

    let sitemap_writer = SiteMapWriter::new(LineWriter::new(file));
    let url_writer = sitemap_writer.start_urlset();

    let mut validated_urls: Vec<String> = url_list
        .iter()
        .map(|url| format_url(url.as_str(), target_url))
        .collect();

    validated_urls.sort();
    validated_urls.dedup();

    match url_writer {
        Ok(mut writer) => {
            for validated_url in validated_urls {
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

/// Build lastmod value vor sitemap entry
fn build_lastmod() -> chrono::DateTime<FixedOffset> {
    return chrono::offset::Utc::now().with_timezone(&FixedOffset::west_opt(0).unwrap());
}

/// Format url to be a valid sitemap url
fn format_url(url: &str, target_url: &str) -> String {
    if url.starts_with("/") {
        return format!("{}{}", target_url, url);
    }

    return format!("{}", url);
}
