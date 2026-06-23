# UI Helpers

A collection of utility functions for common UI tasks in Geode.

### Functions
- `ui::open_mods_list()`
- `ui::create_default_logo() -> Option<Node>`
- `ui::create_server_mod_logo(mod_id: &str) -> Option<Node>`

### Example
```rust
use rustapi::bindings::geode::ui;

#[no_mangle]
pub extern "C" fn my_ui_func() {
    ui::open_mods_list();
    
    if let Some(logo) = ui::create_server_mod_logo("axiom.rustapi") {
        // do something with the logo node
    }
}
```
