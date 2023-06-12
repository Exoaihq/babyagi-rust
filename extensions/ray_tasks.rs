use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use rayon::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task_name: String,
}

struct CooperativeTaskListStorage {
    name: String,
    tasks: Arc<Mutex<VecDeque<Task>>>,
    task_id_counter: Arc<Mutex<usize>>,
}

impl CooperativeTaskListStorage {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tasks: Arc::new(Mutex::new(VecDeque::new())),
            task_id_counter: Arc::new(Mutex::new(0)),
        }
    }

    fn append(&self, task: Task) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push_back(task);
    }

    fn replace(&self, tasks: Vec<Task>) {
        let mut current_tasks = self.tasks.lock().unwrap();
        *current_tasks = VecDeque::from(tasks);
    }

    fn pop_left(&self) -> Option<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.pop_front()
    }

    fn is_empty(&self) -> bool {
        let tasks = self.tasks.lock().unwrap();
        tasks.is_empty()
    }

    fn next_task_id(&self) -> usize {
        let mut task_id_counter = self.task_id_counter.lock().unwrap();
        *task_id_counter += 1;
        *task_id_counter
    }

    fn get_task_names(&self) -> Vec<String> {
        let tasks = self.tasks.lock().unwrap();
        tasks.iter().map(|t| t.task_name.clone()).collect()
    }
}