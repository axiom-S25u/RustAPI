# Time Utilities

get monotonic or unix time.

### Functions
- `time::now() -> f64` (seconds since mod loaded)
- `time::unix() -> f64` (unix epoch timestamp in seconds)

### example

#### rust side

```rust
use rustapi::bindings::geode::time;

let current_sec = time::now();
let epoch_sec = time::unix();
println!("Monotonic: {}, Unix: {}", current_sec, epoch_sec);
```

#### cpp side

you dont need cpp side because time helpers fetch timestamps directly in rust

