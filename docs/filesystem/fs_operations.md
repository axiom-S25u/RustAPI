# Filesystem Operations

helpers to manage folders and files.

### Functions
- `exists(path: &Path) -> bool`
- `list(path: &Path) -> Result<Vec<String>, String>`
- `mkdir(path: &Path) -> Result<(), String>`
- `remove(path: &Path) -> Result<(), String>`

### example

#### rust side

```rust
use rustapi::bindings::geode::fs;
use std::path::Path;

let dir = Path::new("my_data_dir");

// Create directory
fs::mkdir(dir).unwrap();

// Verify directory exists
if fs::exists(dir) {
    // List directory contents
    let files = fs::list(dir).unwrap();
    println!("Found {} files", files.len());
    
    // Clean up
    fs::remove(dir).unwrap();
}
```

#### cpp side

you dont need cpp side directory and file actions are all handled in rust

