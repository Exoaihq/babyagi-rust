```rust
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use chrono::prelude::*;

fn main() {
    let mut tasks_submitted = false;
    let mut task_list = vec![
        Task { status: String::from("incomplete") },
        Task { status: String::from("incomplete") },
        Task { status: String::from("incomplete") },
    ];

    loop {
        if !tasks_submitted {
            for task in &mut task_list {
                if task.status == "incomplete" {
                    task.status = String::from("complete");
                    tasks_submitted = true;
                }
            }
        }

        if !tasks_submitted && task_list.iter().all(|task| task.status == "complete") {
            break;
        }

        thread::sleep(Duration::from_secs(5));
    }

    println!("\x1b[96m\x1b[1m\n*****SAVING FILE...*****\n\x1b[0m\x1b[0m");
    let dt = Local::now().format("%d_%m_%Y_%H_%M_%S");
    let file_name = format!("output/output_{}.txt", dt);
    let mut file = File::create(file_name).expect("Unable to create file");
    let session_summary = "Session summary";
    file.write_all(session_summary.as_bytes()).expect("Unable to write data");
    println!("...file saved.");
    println!("END");
}

struct Task {
    status: String,
}
```
