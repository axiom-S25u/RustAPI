# Intro to RustAPI

so basically rustapi lets you write geode mods in rust.

why do this? mostly because c++ is annoying with pointers, leaks, and compile times. with rust you get cargo so adding libraries is super easy, plus modern features like pattern matching.

prerequisites:
- rustup (rust toolkit)
- geode cli
- cmake (3.22+)
- clang or msvc compiler (c++23 support)

### example

#### cpp side

you need a basic cpp loader to start the rust code

```cpp
#include <Geode/Geode.hpp>
#include <RustAPI.hpp>

extern "C" {
    void my_mod_init();
}

$on_mod(Loaded) {
    my_mod_init();
}
```

#### rust side

write the initialization function in rust

```rust
use rustapi::bindings::geode::log_info;

#[no_mangle]
pub extern "C" fn my_mod_init() {
    log_info("Hello from Rust!");
}
```

