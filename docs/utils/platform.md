# Platform Detection

check which platform the mod is currently running on.

### Functions
- `platform::is_windows() -> bool`
- `platform::is_macos() -> bool`
- `platform::is_ios() -> bool`
- `platform::is_android() -> bool`
- `platform::get_string() -> String`
- `platform::is_wine() -> bool`
### Example
```rust
use rustapi::bindings::geode::platform;

fn setup() {
    if platform::is_windows() {
        println!("running on windows");
    } else if platform::is_macos() {
        println!("running on mac");
    } else if platform::is_android() {
        println!("running on android");
    }
}
```

### cpp side

you dont need cpp side because platform checks run directly on the rust side

