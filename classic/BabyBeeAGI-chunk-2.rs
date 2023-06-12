```rust
extern crate reqwest;
use std::collections::HashMap;
use std::time::Duration;
use std::thread;
use serde_json::Value;

const OBJECTIVE: &str = "YOUR_OBJECTIVE";
const YOUR_FIRST_TASK: &str = "YOUR_FIRST_TASK";

fn main() {
    let mut task_list = Vec::new();
    let mut task_id_counter = 1;

    let first_task = Task {
        id: task_id_counter,
        task: String::from(YOUR_FIRST_TASK),
        tool: String::from("text-completion"),
        dependent_task_id: None,
        status: String::from("incomplete"),
        result: String::from(""),
        result_summary: String::from(""),
    };

    task_list.push(first_task);
    task_id_counter += 1;

    println!("\x1b[96m\x1b[1m\n*****OBJECTIVE*****\n\x1b[0m\x1b[0m");
    println!("{}", OBJECTIVE);

    while task_list.iter().any(|task| task.status == "incomplete") {
        let incomplete_tasks: Vec<&Task> = task_list
            .iter()
            .filter(|task| task.status == "incomplete")
            .collect();

        if !incomplete_tasks.is_empty() {
            let task = &incomplete_tasks[0];
            let task_index = task_list
                .iter()
                .position(|t| t.id == task.id)
                .unwrap();

            execute_task(&mut task_list[task_index], &mut task_list, OBJECTIVE);

            println!("\x1b[95m\x1b[1m\n*****TASK LIST*****\n\x1b[0m");
            for task in &task_list {
                let dependent_task = match task.dependent_task_id {
                    Some(id) => format!("\x1b[31m<dependency: #{}>\x1b[0m", id),
                    None => String::from(""),
                };
                let status_color = if task.status == "complete" {
                    "\x1b[32m"
                } else {
                    "\x1b[31m"
                };
                println!(
                    "\x1b[1m{}\x1b[0m: {} {}[{}]\x1b[0m \x1b[93m[{}] {}",
                    task.id, task.task, status_color, task.status, task.tool, dependent_task
                );
            }
            let session_summary = update_session_summary(&task_list);
            println!("\x1b[93m\x1b[1m\n*****SESSION SUMMARY*****\n\x1b[0m\x1b[0m");
            println!("{}", session_summary);
        }

        thread::sleep(Duration::from_secs(1));
    }

    if task_list.iter().all(|task| task.status != "incomplete") {
        println!("\x1b[92m\x1b[1m\n*****ALL TASKS COMPLETED*****\n\x1b[0m\x1b[0m");
        for task in task_list {
            println!("ID: {}, Task: {}, Result: {}", task.id, task.task, task.result);
        }
    }
}

#[derive(Clone)]
struct Task {
    id: usize,
    task: String,
    tool: String,
    dependent_task_id: Option<usize>,
    status: String,
    result: String,
    result_summary: String,
}

fn execute_task(task: &mut Task, task_list: &mut Vec<Task>, objective: &str) {
    task.status = String::from("complete");
    task.result = String::from("Task result");
    task.result_summary = String::from("Task result summary");
}

fn update_session_summary(task_list: &[Task]) -> String {
    let prompt = format!(
        "Objective: {}\n\nTask list:\n",
        OBJECTIVE
    );

    let mut tasks_text = String::new();
    for task in task_list {
        tasks_text.push_str(&format!(
            "Task {}: {}\nResult: {}\nResult summary: {}\n\n",
            task.id, task.task, task.result, task.result_summary
        ));
    }

    let prompt = format!("{}{}", prompt, tasks_text);

    let response = openai_completion(&prompt);
    response.trim().to_string()
}

fn openai_completion(prompt: &str) -> String {
    let api_key = "your_openai_api_key";
    let url = "https://api.openai.com/v1/engines/text-davinci-003/completions";
    let client = reqwest::blocking::Client::new();

    let mut data = HashMap::new();
    data.insert("prompt", prompt);
    data.insert("temperature", "0.5");
    data.insert("max_tokens", "200");
    data.insert("top_p", "1");
    data.insert("frequency_penalty", "0");
    data.insert("presence_penalty", "0");

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&data)
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();

    response["choices"][0]["text"].as_str().unwrap().to_string()
}
```