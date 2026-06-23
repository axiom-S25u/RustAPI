# Base64

encode and decode base64.

### Functions
- `encode(data: &[u8]) -> String`
- `decode(data: &str) -> Result<Vec<u8>, String>`
- `decode_string(data: &str) -> Result<String, String>`

### example

#### rust side

```rust
use rustapi::bindings::geode::base64;

let encoded = base64::encode(b"hello world");
let decoded = base64::decode_string(&encoded).unwrap();
assert_eq!(decoded, "hello world");
```

#### cpp side

you dont need cpp side because base64 operations are done in rust

