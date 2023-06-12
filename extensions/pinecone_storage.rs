use std::collections::HashMap;
use std::error::Error;
use std::ffi::CString;
use std::os::raw::c_char;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct PineconeResultsStorage {
    openai_api_key: String,
    pinecone_api_key: String,
    pinecone_environment: String,
    llm_model: String,
    llama_model_path: String,
    results_store_name: String,
    objective: String,
    namespace: String,
}

impl PineconeResultsStorage {
    pub fn new(
        openai_api_key: &str,
        pinecone_api_key: &str,
        pinecone_environment: &str,
        llm_model: &str,
        llama_model_path: &str,
        results_store_name: &str,
        objective: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let namespace = objective.chars().filter(|c| c.is_ascii()).collect::<String>();

        let storage = PineconeResultsStorage {
            openai_api_key: openai_api_key.to_string(),
            pinecone_api_key: pinecone_api_key.to_string(),
            pinecone_environment: pinecone_environment.to_string(),
            llm_model: llm_model.to_string(),
            llama_model_path: llama_model_path.to_string(),
            results_store_name: results_store_name.to_string(),
            objective: objective.to_string(),
            namespace,
        };

        storage.init()?;

        Ok(storage)
    }

    fn init(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Initialize Pinecone and OpenAI clients
        // Rust currently does not have official Pinecone and OpenAI libraries.
        // You will need to implement the necessary API calls using an HTTP client like reqwest.
        Ok(())
    }

    pub fn add(&self, task: HashMap<String, String>, result: &str, result_id: i32) -> Result<(), Box<dyn Error>> {
        let vector = self.get_embedding(result)?;
        // TODO: Upsert the result into Pinecone index
        Ok(())
    }

    pub fn query(&self, query: &str, top_results_num: i32) -> Result<Vec<String>, Box<dyn Error>> {
        let query_embedding = self.get_embedding(query)?;
        // TODO: Query Pinecone index and return the sorted results
        Ok(vec![])
    }

    fn get_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
        let text = text.replace("\n", " ");

        if self.llm_model.starts_with("llama") {
            // TODO: Implement Llama embedding in Rust
            // Rust currently does not have a Llama library.
            // You will need to implement the necessary Llama embedding functionality.
            Ok(vec![])
        } else {
            self.get_openai_embedding(text)
        }
    }

    fn get_openai_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let url = "https://api.openai.com/v1/embeds";
        let mut payload = HashMap::new();
        payload.insert("input", text);
        payload.insert("model", "text-embedding-ada-002");

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.openai_api_key))
            .json(&payload)
            .send()?;

        let response_json: Value = response.json()?;
        let embedding = response_json["data"][0]["embedding"]
            .as_array()
            .ok_or("Failed to parse embedding")?
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect::<Vec<f32>>();

        Ok(embedding)
    }
}