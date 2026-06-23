# Filesystem Overview

rustapi has sandboxed filesystem helpers. you can only access paths under geometry dash folders.

### example

#### rust side

```rust
use rustapi::bindings::geode::fs;
use std::path::Path;

if fs::exists(Path::new("config.json")) {
    println!("File exists!");
}
```

#### cpp side

you dont need cpp side here because sandboxed file checks are done in rust

