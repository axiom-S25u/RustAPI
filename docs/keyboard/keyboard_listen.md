# Keyboard Listeners

capture key presses, releases, and repeats.

### API
- `listen<F>(key: i32, priority: i32, callback: F) -> KeyboardListenerHandle`
  - `key`: key code to filter, or `-1` for all keys.
  - `priority`: event priority order.
  - `callback`: `FnMut(&KeyboardInputData) -> bool`. return `true` to block the key from other mods/game, or `false` to let it propagate.

### Lifetime
the listener is disconnected when the returned `KeyboardListenerHandle` is dropped.

### Example
```rust
use rustapi::bindings::geode::keyboard;

// listen for space (key 32)
let _handle = keyboard::listen(32, 0, |data| {
    println!("space event action: {}", data.action);
    true // block it
});
```

### cpp side

you dont need cpp side because the rust listener hooks geode key event dispatcher and converts it to rust types automatically

