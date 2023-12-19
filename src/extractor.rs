use headless_chrome::Browser;
use log::info;
use scraper::{Html, Selector};
use std::error::Error;
use url::Url;

/// Extract urls from page
pub fn extract_urls(target_url: &Url) -> Result<Vec<String>, Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    tab.navigate_to(target_url.as_str())?;
    _ = tab.wait_until_navigated();

    let content = tab.get_content()?;
    _ = tab.close(true);

    let filtered_stuff = scrape_urls(&target_url.as_str(), &content);
    return Ok(filtered_stuff);
}

/// Scrape urls from document
fn scrape_urls(target_url: &str, cont: &str) -> Vec<String> {
    let document = Html::parse_document(cont);
    let selector = Selector::parse("a").unwrap();

    let selection = document.select(&selector);

    let mut filtered_stuff: Vec<String> = vec![];

    for selected_iter in selection.into_iter() {
        let href = selected_iter.value().attr("href");

        match href {
            Some(href_value) => {
                if (href_value.contains(&target_url) || href_value.starts_with("/"))
                    && !filtered_stuff.contains(&href_value.to_string())
                {
                    info!("Link extracted: {}", href_value);
                    filtered_stuff.push(href_value.to_string());
                }
            }
            None => info!("None links could be found!"),
        }
    }

    filtered_stuff.sort();
    return filtered_stuff;
}
