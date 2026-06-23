# Keybinds

parse or format keybinds.

### Functions
- `from_string(s: &str) -> Result<serde_json::Value, String>`
- `to_string(key: i32, modifiers: i32) -> String`

### example

#### rust side

```rust
use rustapi::bindings::geode::keybind;

let keybind_str = keybind::to_string(32, 1); // returns "Shift + Space"
```

#### cpp side

you dont need cpp side because parsing and formatting keybinds is done inside rust

