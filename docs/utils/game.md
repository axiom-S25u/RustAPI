# Game Utilities

Utilities for interacting with the Geode game lifecycle.

### Functions
- `game::exit()`
- `game::restart()`
- `game::launch_loader_uninstaller(delete_save: bool)`

### Example
```rust
use rustapi::bindings::geode::game;

#[no_mangle]
pub extern "C" fn my_rage_quit_func() {
    // bye bye
    game::exit();
}
```
