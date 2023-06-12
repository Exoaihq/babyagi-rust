```rust
use reqwest::header::USER_AGENT;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::collections::HashMap;
use std::time::Duration;

fn simplify_search_results(search_results: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    search_results
        .into_iter()
        .map(|result| {
            let mut simplified_result = HashMap::new();
            simplified_result.insert("position".to_string(), result.get("position").unwrap().to_string());
            simplified_result.insert("title".to_string(), result.get("title").unwrap().to_string());
            simplified_result.insert("link".to_string(), result.get("link").unwrap().to_string());
            simplified_result.insert("snippet".to_string(), result.get("snippet").unwrap().to_string());
            simplified_result
        })
        .collect()
}

fn web_scrape_tool(url: &str, task: &str) -> Option<String> {
    let content = fetch_url_content(url)?;
    let text = extract_text(&content);
    println!("\x1B[90m\x1B[3mScrape completed. Length: {}. Now extracting relevant info...\x1B[0m", text.len());
    let info = extract_relevant_info(OBJECTIVE, &text[..5000], task);
    let links = extract_links(&content);

    Some(info)
}

fn fetch_url_content(url: &str) -> Option<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let response = client.get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36")
        .send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                Some(resp.text().unwrap())
            } else {
                println!("Error while fetching the URL: {}", resp.status());
                None
            }
        }
        Err(e) => {
            println!("Error while fetching the URL: {}", e);
            None
        }
    }
}

fn extract_links(content: &str) -> Vec<String> {
    let document = Document::from(content);
    document.find(Attr("href", ()).and(Name("a")))
        .filter_map(|n| n.attr("href"))
        .filter(|link| link.starts_with("http") || link.starts_with("https"))
        .map(|link| link.to_string())
        .collect()
}

fn extract_text(content: &str) -> String {
    let document = Document::from(content);
    document.text()
}

fn extract_relevant_info(objective: &str, large_string: &str, task: &str) -> String {
    let chunk_size = 3000;
    let overlap = 500;
    let mut notes = String::new();

    for i in (0..large_string.len()).step_by(chunk_size - overlap) {
        let chunk = &large_string[i..i + chunk_size];

        // Add your GPT-3.5-turbo API call here
        // ...
        // ...

        notes.push_str(&response);
        notes.push_str(". ");
    }

    notes
}
```
Note: This Rust code assumes you have the following dependencies in your `Cargo.toml` file:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
select = "0.5"
```