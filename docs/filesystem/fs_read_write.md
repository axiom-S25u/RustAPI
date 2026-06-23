# Filesystem Read & Write

simple helpers to read and write text files.

### Functions
- `read(path: &Path) -> Result<String, String>`
- `write(path: &Path, data: &str) -> Result<(), String>`

### Example
```rust
use rustapi::bindings::geode::fs;
use std::path::Path;

let file = Path::new("config.json");
fs::write(file, "{\"enabled\": true}").unwrap();

let text = fs::read(file).unwrap();
```

### cpp side

you dont need cpp side because these are just rust wrappers for sandbox file read and write

