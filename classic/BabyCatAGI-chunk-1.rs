Here's the converted Rust code:

```rust
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Error;
use select::document::Document;
use select::predicate::{Attr, Name};
use std::time::Duration;

fn main() {
    let url = "https://example.com";
    let content = fetch_url_content(url);
    let links = extract_links(&content);
    let text = extract_text(&content);
    println!("Links: {:?}", links);
    println!("Text: {}", text);
}

fn fetch_url_content(url: &str) -> String {
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36";
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()
        .unwrap();

    match client.get(url).send() {
        Ok(response) => {
            if response.status().is_success() {
                response.text().unwrap_or_default()
            } else {
                println!("Error while fetching the URL: {}", response.status());
                String::new()
            }
        }
        Err(e) => {
            println!("Error while fetching the URL: {}", e);
            String::new()
        }
    }
}

fn extract_links(content: &str) -> Vec<String> {
    let document = Document::from(content);
    document
        .find(Name("a").and(Attr("href", ())))
        .filter_map(|n| n.attr("href"))
        .filter(|link| link.starts_with("http://") || link.starts_with("https://"))
        .map(String::from)
        .collect()
}

fn extract_text(content: &str) -> String {
    let document = Document::from(content);
    document.text()
}
```

This Rust code performs the same functionality as the original Python code. It fetches the content of a URL, extracts links and text from the content, and prints the results.