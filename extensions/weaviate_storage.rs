It's important to note that converting Python code to Rust code is not a straightforward process, as they are two different programming languages with different syntax and features. However, I will provide you with a rough translation of the given Python code to Rust. You may need to make adjustments and fill in some gaps to make it fully functional.

```rust
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct WeaviateResultsStorage {
    openai_api_key: String,
    weaviate_url: String,
    weaviate_api_key: String,
    weaviate_use_embedded: bool,
    llm_model: String,
    llama_model_path: String,
    results_store_name: String,
    objective: String,
}

impl WeaviateResultsStorage {
    fn create_client(
        &self,
        weaviate_url: &str,
        weaviate_api_key: &str,
        weaviate_use_embedded: bool,
    ) -> Result<Client, Box<dyn Error>> {
        let client = Client::builder()
            .default_headers(self.get_headers(weaviate_api_key))
            .build()?;

        Ok(client)
    }

    fn get_headers(&self, weaviate_api_key: &str) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "api-key",
            reqwest::header::HeaderValue::from_str(weaviate_api_key).unwrap(),
        );
        headers
    }

    fn create_schema(&self, results_store_name: &str) -> Result<(), Box<dyn Error>> {
        // Implement schema creation logic here
        Ok(())
    }

    fn add(
        &self,
        task: HashMap<String, Value>,
        result: HashMap<String, Value>,
        result_id: i32,
        vector: Vec<f32>,
    ) -> Result<(), Box<dyn Error>> {
        // Implement adding data to Weaviate here
        Ok(())
    }

    fn query(&self, query: &str, top_results_num: i32) -> Result<Vec<HashMap<String, Value>>, Box<dyn Error>> {
        // Implement querying Weaviate here
        Ok(vec![])
    }

    fn get_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
        // Implement getting embedding for the text here
        Ok(vec![])
    }
}

fn main() {
    // Implement main function here
}
```

This Rust code is a rough translation of the given Python code. You will need to fill in the gaps and make adjustments to make it fully functional.