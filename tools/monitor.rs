```rust
use std::io::{stdout, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

use crossterm::{cursor, execute, terminal};
use crossterm::terminal::{Clear, ClearType};

mod extensions {
    pub mod ray_objectives {
        pub struct CooperativeObjectivesListStorage;

        impl CooperativeObjectivesListStorage {
            pub fn get_objective_names(&self) -> Vec<String> {
                // Implement the logic to get objective names here.
                vec![]
            }
        }
    }

    pub mod ray_tasks {
        pub struct CooperativeTaskListStorage(pub String);

        impl CooperativeTaskListStorage {
            pub fn get_task_names(&self) -> Vec<String> {
                // Implement the logic to get task names here.
                vec![]
            }
        }
    }
}

fn print_buffer(lines: &[String]) {
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

    for line in lines {
        writeln!(stdout, "{}", line).unwrap();
    }

    stdout.flush().unwrap();
}

fn main() {
    let objectives = extensions::ray_objectives::CooperativeObjectivesListStorage;

    loop {
        let objectives_list = objectives.get_objective_names();
        let mut buffer = vec![];

        if objectives_list.is_empty() {
            buffer.push("No objectives".to_string());
        }

        for objective in objectives_list {
            buffer.push("-----------------".to_string());
            buffer.push(format!("Objective: {}", objective));
            buffer.push("-----------------".to_string());
            let tasks = extensions::ray_tasks::CooperativeTaskListStorage(objective);
            let tasks_list = tasks.get_task_names();
            buffer.push("Tasks:".to_string());
            for t in tasks_list {
                buffer.push(format!(" * {}", t));
            }
            buffer.push("-----------------".to_string());
        }

        print_buffer(&buffer);
        thread::sleep(Duration::from_secs(30));
    }
}
```

This is a Rust version of the provided Python code. Note that you'll need to implement the logic for getting objective and task names in the `get_objective_names` and `get_task_names` methods.