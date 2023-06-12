```rust
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

struct CooperativeObjectivesListStorageActor {
    objectives: Arc<Mutex<VecDeque<String>>>,
}

impl CooperativeObjectivesListStorageActor {
    fn new() -> Self {
        CooperativeObjectivesListStorageActor {
            objectives: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    fn append(&mut self, objective: String) {
        let mut objectives = self.objectives.lock().unwrap();
        if !objectives.contains(&objective) {
            objectives.push_back(objective);
        }
    }

    fn is_empty(&self) -> bool {
        let objectives = self.objectives.lock().unwrap();
        objectives.is_empty()
    }

    fn get_objective_names(&self) -> Vec<String> {
        let objectives = self.objectives.lock().unwrap();
        objectives.iter().cloned().collect()
    }
}

struct CooperativeObjectivesListStorage {
    actor: CooperativeObjectivesListStorageActor,
}

impl CooperativeObjectivesListStorage {
    fn new() -> Self {
        CooperativeObjectivesListStorage {
            actor: CooperativeObjectivesListStorageActor::new(),
        }
    }

    fn append(&mut self, objective: String) {
        self.actor.append(objective);
    }

    fn is_empty(&self) -> bool {
        self.actor.is_empty()
    }

    fn get_objective_names(&self) -> Vec<String> {
        self.actor.get_objective_names()
    }
}
```
Please note that this Rust code does not use the Ray framework, as there is no direct equivalent in Rust. Instead, it uses the `rayon` crate for parallelism. You may need to add the `rayon` crate to your `Cargo.toml` file:

```toml
[dependencies]
rayon = "1.5.1"
```