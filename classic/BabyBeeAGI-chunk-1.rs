Here's the Rust version of the code:

```rust
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut task_list = vec![
        // Add your tasks here
    ];

    let current_task_id = 0; // Replace with the actual task ID
    let result = "Some result"; // Replace with the actual result

    let original_task_list = task_list.clone();
    let minified_task_list: Vec<HashMap<String, Value>> = task_list
        .iter()
        .map(|task| {
            task.iter()
                .filter(|(k, _)| k != "result")
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        })
        .collect();

    // Implement your AI logic here

    for (updated_task, original_task) in task_list.iter_mut().zip(original_task_list.iter()) {
        if let Some(result) = original_task.get("result") {
            updated_task.insert("result".to_string(), result.clone());
        }
    }

    task_list[current_task_id as usize]
        .insert("result".to_string(), Value::String(result.to_string()));

    println!("{:?}", task_list);

    Ok(())
}
```

Please note that you'll need to add your tasks to the `task_list` variable and replace the `current_task_id` and `result` variables with the actual values. Also, you need to implement the AI logic in the appropriate section of the code. Don't forget to add the `serde` and `serde_json` crates to your `Cargo.toml` file for JSON handling.