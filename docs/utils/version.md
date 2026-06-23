# Version Parsing

helpers for semver checks.

### Functions
- `parse(version_str: &str) -> Result<String, String>`
- `compare(a: &str, b: &str) -> i32`
- `matches(constraint: &str, version: &str) -> bool`

### example

#### rust side

```rust
use rustapi::bindings::geode::version;

let parsed = version::parse("2.1.0-alpha.1").unwrap();
let cmp = version::compare("1.0.0", "2.0.0"); // returns -1
let fits = version::matches(">=1.5.0", "2.0.0"); // returns true
```

#### cpp side

you dont need cpp side because semver checks are done in rust

