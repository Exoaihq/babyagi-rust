```rust
extern crate reqwest;
extern crate serde_json;

use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

static OBJECTIVE: &str = "Research Untapped Capital and provide a summary of their investment thesis, portfolio companies, and team members.";

async fn openai_chat_completion(
    model: &str,
    messages: Vec<HashMap<&str, &str>>,
    temperature: f32,
    max_tokens: u32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
) -> Result<Value, Box<dyn Error>> {
    let api_key = "your_openai_api_key";
    let url = "https://api.openai.com/v1/engines/davinci-codex/completions";

    let client = reqwest::Client::new();
    let mut request_body = HashMap::new();
    request_body.insert("model", model);
    request_body.insert("messages", &messages);
    request_body.insert("temperature", &temperature);
    request_body.insert("max_tokens", &max_tokens);
    request_body.insert("top_p", &top_p);
    request_body.insert("frequency_penalty", &frequency_penalty);
    request_body.insert("presence_penalty", &presence_penalty);

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let mut response_body = String::new();
    response.read_to_string(&mut response_body)?;

    let result: Value = serde_json::from_str(&response_body)?;
    Ok(result)
}

fn main() {
    let messages = vec![
        hashmap! {
            "role" => "system",
            "content" => "You are a task creation AI.",
        },
        hashmap! {
            "role" => "user",
            "content" => OBJECTIVE,
        },
    ];

    let response = openai_chat_completion(
        "gpt-4",
        messages,
        0.0,
        1500,
        1.0,
        0.0,
        0.0,
    );

    match response {
        Ok(result) => {
            let task_list: Value = result["choices"][0]["message"]["content"].clone();
            println!("Task list: {}", task_list);
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

Please note that you'll need to replace `your_openai_api_key` with your actual OpenAI API key. Also, make sure to add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
```