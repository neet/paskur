use scraper::{Element, Html, Selector};
use url::{form_urlencoded::Serializer, Url};

pub async fn search(title: &str) -> Result<String, reqwest::Error> {
    let base_url = "https://search.yahoo.co.jp/search";
    let url = Url::parse(base_url).unwrap();

    let params = [
        ("p", &title),
        ("vs", &"www.y-history.net/appendix"),
        ("fr", &"ysin"),
        ("ei", &"utf-8"),
    ];

    let search = &Serializer::new(String::new())
        .extend_pairs(&params)
        .finish();

    let search_url = url
        .join(&("?".to_string() + search))
        .expect("Failed to parse URL");

    let resp = reqwest::Client::new()
        .get(search_url)
        .header("User-Agent", "Chrome/114.0.0.0")
        .send()
        .await?;

    let body = resp.text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("h3").unwrap();

    let element = document
        .select(&selector)
        .next()
        .expect("Failed to find the element");

    let parent_element = element
        .parent_element()
        .expect("Failed to find the parent element");

    let href = parent_element
        .value()
        .attr("href")
        .expect("Failed to find the href");

    Ok(href.to_string())
}
