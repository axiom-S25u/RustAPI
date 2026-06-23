# Random Utilities

Utilities for generating random UUIDs and strings via Geode's C++ random number generator.

### Functions
- `random::generate_uuid() -> String`
- `random::generate_hex_string(length: i32) -> String`
- `random::generate_alphanumeric_string(length: i32) -> String`
- `random::generate_string(length: i32, chars: &str) -> String`

### Example
```rust
use rustapi::bindings::geode::random;

#[no_mangle]
pub extern "C" fn test_random() {
    let uuid = random::generate_uuid();
    println!("random uuid: {}", uuid);
    
    let hex = random::generate_hex_string(16);
    println!("hex: {}", hex);
}
```
