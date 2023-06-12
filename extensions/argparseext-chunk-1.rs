```rust
use std::env;
use std::process;

fn main() {
    let no_initial_task = "No initial task specified or found in environment.\n";
    let red_bold = "\x1b[91m\x1b[1m";
    let reset = "\x1b[0m\x1b[0m";

    let objective = match env::var("OBJECTIVE") {
        Ok(value) => value,
        Err(_) => {
            println!("{}{}{}", red_bold, no_initial_task, reset);
            print_help();
            process::exit(1);
        }
    };

    // Add the rest of the variables and their error handling here

    // Return the values
    // (objective, initial_task, llm_model, dotenv_extensions, instance_name, cooperative_mode, join_existing_objective)
}

fn print_help() {
    // Add the help message printing logic here
}
```
Please note that you will need to add the rest of the variables and their error handling, as well as the help message printing logic in the `print_help()` function.