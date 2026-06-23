# Thread Utilities

Utilities for interacting with the Geode thread names.

### Functions
- `thread::get_default_name() -> String`
- `thread::get_name() -> String`
- `thread::set_name(name: &str)`

### Example
```rust
use rustapi::bindings::geode::thread;

#[no_mangle]
pub extern "C" fn test_thread() {
    let name = thread::get_name();
    println!("current thread: {}", name);
    thread::set_name("my_custom_rust_thread");
}
```
