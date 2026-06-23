# Settings & Saved Values

### Settings
read settings or listen for changes.

- `get_setting_value(&self, key: &str) -> Result<serde_json::Value, String>`
- `has_setting(&self, key: &str) -> bool`
- `listen_for_setting_changes<F>(&self, key: &str, callback: F)`
- `listen_for_all_setting_changes<F>(&self, callback: F)`

### Saved Values
saved values are variables saved across game restarts.

- `get_saved_value(&self, key: &str) -> Result<serde_json::Value, String>`
- `set_saved_value(&self, key: &str, value: &serde_json::Value) -> Result<(), String>`

### example

#### rust side

```rust
use rustapi::bindings::geode::Mod;
use serde_json::json;

let my_mod = Mod::get();

// Reading a setting
if let Ok(value) = my_mod.get_setting_value("some-bool-setting") {
    let is_enabled = value.as_bool().unwrap_or(false);
}

// Listening for setting changes
my_mod.listen_for_setting_changes("some-bool-setting", |val| {
    println!("Setting changed: {:?}", val);
});

// Using saved values
my_mod.set_saved_value("my-score", &json!(420)).unwrap();
let score = my_mod.get_saved_value("my-score").unwrap();
```

#### cpp side

you dont need cpp side settings and saved values are managed directly in rust using the mod instance

