```rust
extern crate tiktoken;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tiktoken::{TokenizedString, Tokenizer};

struct DefaultResultsStorage;

impl DefaultResultsStorage {
    fn new() -> Self {
        DefaultResultsStorage
    }
}

struct SingleTaskListStorage {
    tasks: VecDeque<std::collections::HashMap<String, String>>,
    task_id_counter: u32,
}

impl SingleTaskListStorage {
    fn new() -> Self {
        SingleTaskListStorage {
            tasks: VecDeque::new(),
            task_id_counter: 0,
        }
    }

    fn append(&mut self, task: std::collections::HashMap<String, String>) {
        self.tasks.push_back(task);
    }

    fn replace(&mut self, tasks: Vec<std::collections::HashMap<String, String>>) {
        self.tasks = tasks.into();
    }

    fn pop_left(&mut self) -> Option<std::collections::HashMap<String, String>> {
        self.tasks.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    fn next_task_id(&mut self) -> u32 {
        self.task_id_counter += 1;
        self.task_id_counter
    }

    fn get_task_names(&self) -> Vec<String> {
        self.tasks
            .iter()
            .map(|t| t.get("task_name").unwrap().to_string())
            .collect()
    }
}

fn limit_tokens_from_string(
    string: &str,
    model: &str,
    limit: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let tokenizer = Tokenizer::new();
    let tokenized_string = TokenizedString::from(tokenizer.tokenize(string));
    let tokens = tokenized_string.tokens;

    if tokens.len() > limit {
        let limited_tokens = tokens[..limit].to_vec();
        let limited_string = tokenizer.detokenize(&limited_tokens);
        Ok(limited_string)
    } else {
        Ok(string.to_string())
    }
}

fn main() {
    let tasks_storage = Arc::new(Mutex::new(SingleTaskListStorage::new()));

    // Example usage of limit_tokens_from_string
    match limit_tokens_from_string("This is a test string", "gpt-3", 5) {
        Ok(result) => println!("Limited string: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```
Please note that this Rust code is a partial conversion of the provided Python code. Some functionalities, like the OpenAI API calls, are not available in Rust and would require additional libraries and implementation. The provided code focuses on the `SingleTaskListStorage` struct and the `limit_tokens_from_string` function.