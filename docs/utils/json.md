# JSON Helpers

helpers using `serde_json`.

### Functions
- `parse(text: &str) -> Result<serde_json::Value, String>`
- `dump(value: &serde_json::Value, indent: Option<usize>) -> Result<String, String>`

### Example
```rust
use rustapi::bindings::geode::json;

let parsed = json::parse(r#"{"val": 123}"#).unwrap();
let val = parsed["val"].as_i64().unwrap();
```

### cpp side

you dont need cpp side because rust uses serde_json and handles ffi conversions to and from geode matjson structures under the hood

