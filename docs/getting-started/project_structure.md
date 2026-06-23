# Project Structure

your mod directory should look like this:

my-mod/
├── CMakeLists.txt
├── Cargo.toml
├── mod.json
└── src/
    ├── main.cpp
    └── lib.rs

### Cargo.toml
make sure you set the library type to staticlib so cmake can link it.

```toml
[lib]
name = "my_mod"
crate-type = ["staticlib"]

[dependencies]
rustapi = { path = "../RustAPI" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### example

#### cpp side

`src/main.cpp`:
```cpp
#include <Geode/Geode.hpp>
#include <RustAPI.hpp>

extern "C" {
    void my_rust_init();
}

$on_mod(Loaded) {
    my_rust_init();
}
```

#### rust side

`src/lib.rs`:
```rust
use rustapi::bindings::geode::log_info;

#[no_mangle]
pub extern "C" fn my_rust_init() {
    log_info("Rust code is initialized successfully");
}
```

