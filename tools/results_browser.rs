```rust
extern crate dotenv;
extern crate openai;
extern crate pinecone;
extern crate cursive;

use std::env;
use dotenv::dotenv;
use openai::{Openai, Embedding};
use pinecone::{Pinecone, Query};
use cursive::Cursive;
use cursive::views::{TextView, ListView, Dialog, LinearLayout, Panel};
use cursive::traits::*;

fn main() {
    dotenv().ok();

    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is missing from .env");
    let pinecone_api_key = env::var("PINECONE_API_KEY").expect("PINECONE_API_KEY environment variable is missing from .env");
    let pinecone_environment = env::var("PINECONE_ENVIRONMENT").expect("PINECONE_ENVIRONMENT environment variable is missing from .env");
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME environment variable is missing from .env");

    let openai = Openai::new(openai_api_key);
    let pinecone = Pinecone::new(pinecone_api_key, pinecone_environment);

    let index = pinecone.index(table_name);

    let mut siv = Cursive::default();

    let objective = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let tasks = query_records(&index, &openai, &objective);

    let list_view = ListView::new()
        .with(tasks.iter().enumerate().map(|(i, task)| {
            let task_name = format!("{}: {}", i + 1, task.name);
            (task_name, move |s| {
                s.pop_layer();
                s.add_layer(Dialog::text(task.result.clone()).title("Result").button("Ok", |s| s.pop_layer()));
            })
        }).collect::<Vec<_>>());

    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(Panel::new(list_view.scrollable().fixed_size((40, 20))))
                .child(TextView::new("Select a task to see the result").scrollable().fixed_size((40, 20))),
        )
        .title("Task List")
        .button("Quit", |s| s.quit()),
    );

    siv.run();
}

fn query_records(index: &pinecone::Index, openai: &Openai, objective: &str) -> Vec<Task> {
    let embedding = get_ada_embedding(openai, objective);
    let query = Query::new(embedding).top_k(1000).include_metadata(true);
    let results = index.query(query).unwrap();

    results.matches.into_iter().map(|task| {
        Task {
            name: task.metadata["task"].to_string(),
            result: task.metadata["result"].to_string(),
        }
    }).collect()
}

fn get_ada_embedding(openai: &Openai, text: &str) -> Vec<f32> {
    let embedding = Embedding::create(openai, vec![text.to_string()], "text-embedding-ada-002").unwrap();
    embedding.data[0].embedding.clone()
}

#[derive(Clone)]
struct Task {
    name: String,
    result: String,
}
```
