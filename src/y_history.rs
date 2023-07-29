use scraper::{Element, Html, Selector};
use url::{form_urlencoded::Serializer, Url};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

pub async fn search(title: &str) -> Result<String, reqwest::Error> {
    let base_url = "https://google.com/search";
    let url = Url::parse(base_url).unwrap();

    let query = title.to_string() + &" site:y-history.net".to_string();
    let params = [("q", &query)];

    let search = &Serializer::new(String::new())
        .extend_pairs(&params)
        .finish();

    let search_url = url
        .join(&("?".to_string() + search))
        .expect("Failed to parse URL");

    let resp = reqwest::Client::new()
        .get(search_url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;

    let body = resp.text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("h3").unwrap();

    let element = document
        .select(&selector)
        .next()
        .expect("Failed to find the first h3 element");

    let parent_element = element
        .parent_element()
        .expect("Failed to find the parent element");

    let href = parent_element
        .value()
        .attr("href")
        .expect("Failed to find the href");

    Ok(href.to_string())
}
