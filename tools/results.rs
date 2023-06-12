```rust
use std::env;
use std::process::Command;
use std::str;
use serde_json::Value;
use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;

fn main() {
    dotenv().ok();

    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is missing from .env");
    let pinecone_api_key = env::var("PINECONE_API_KEY").expect("PINECONE_API_KEY environment variable is missing from .env");
    let pinecone_environment = env::var("PINECONE_ENVIRONMENT").expect("PINECONE_ENVIRONMENT environment variable is missing from .env");
    let pinecone_table_name = env::var("TABLE_NAME").expect("TABLE_NAME environment variable is missing from .env");

    let objective = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let objective = if objective.is_empty() {
        env::var("OBJECTIVE").expect("OBJECTIVE environment variable is missing from .env")
    } else {
        objective
    };

    let query = get_ada_embedding(&openai_api_key, &objective);
    let retrieved_tasks = query_records(&pinecone_api_key, &pinecone_table_name, &query);

    for task in retrieved_tasks {
        println!("{}", task);
    }
}

fn get_ada_embedding(api_key: &str, text: &str) -> Vec<f32> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());

    let response = client.post("https://api.openai.com/v1/embeds")
        .headers(headers)
        .json(&json!({
            "input": text,
            "model": "text-embedding-ada-002"
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let embedding = json["data"][0]["embedding"].as_array().unwrap();
    embedding.iter().map(|v| v.as_f64().unwrap() as f32).collect()
}

fn query_records(api_key: &str, table_name: &str, query: &[f32]) -> Vec<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());

    let response = client.post(&format!("https://{}.pinecone.io/v1/query", table_name))
        .headers(headers)
        .json(&json!({
            "query": query,
            "top_k": 1000,
            "include_metadata": true
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let matches = json["matches"].as_array().unwrap();

    matches.iter().map(|m| {
        format!("{}:\n{}\n------------------",
            m["metadata"]["task"].as_str().unwrap(),
            m["metadata"]["result"].as_str().unwrap()
        )
    }).collect()
}
```
