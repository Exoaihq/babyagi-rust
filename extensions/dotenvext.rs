```toml
# Add this to your Cargo.toml
[dependencies]
dotenv = "0.15.0"
```

```rust
extern crate dotenv;

use dotenv::from_filename;
use std::path::Path;

fn load_dotenv_extensions(dotenv_files: Vec<&str>) {
    for dotenv_file in dotenv_files {
        from_filename(Path::new(dotenv_file)).ok();
    }
}
```