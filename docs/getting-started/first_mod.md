# Your First Mod

here is how to make a basic mod that shows a welcome popup on game start.

## Step 1: Create the C++ Loader Hook
you need a c++ file to load rust and hook the menu layer.

```cpp
#include <Geode/Geode.hpp>
#include <Geode/modify/MenuLayer.hpp>
#include <RustAPI.hpp>

using namespace geode::prelude;

extern "C" {
    void my_mod_init();
    void my_mod_on_menu();
}

$on_mod(Loaded) {
    my_mod_init();
}

struct $modify(MyMenuLayer, MenuLayer) {
    bool init() {
        if (!MenuLayer::init()) return false;
        my_mod_on_menu();
        return true;
    }
};
```

## Step 2: Implement Logic in Rust
now write the actual logic in `src/lib.rs`.

> [!IMPORTANT]
> **Don't forget `#[no_mangle]` and `pub extern "C"`!**
> Because you are calling Rust functions from a C++ loader, you **must** use the `#[no_mangle]` attribute to prevent the Rust compiler from scrambling the function name.
> You must also use `pub extern "C"` to ensure the function uses the correct C-ABI calling convention, otherwise the C++ side won't be able to find or execute your code!

```rust
use rustapi::bindings::geode::{log_info, create_quick_popup};

#[no_mangle]
pub extern "C" fn my_mod_init() {
    log_info("mod loaded from rust yayyyyyy");
}

#[no_mangle]
pub extern "C" fn my_mod_on_menu() {
    create_quick_popup(
        "hi from rust",
        "this popup was spawned by rust i guess",
        "ok",
        "cool",
        |clicked_btn2| {
            log_info(&format!("User clicked button 2: {}", clicked_btn2));
        }
    );
}
```

### example

cpp side and rust side code examples are shown above in step 1 and step 2