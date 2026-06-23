# Keyboard Modifiers & Codes

### KeyboardInputData Fields
- `key: i32`
- `action: i32` (0 = Release, 1 = Press, 2 = Repeat)
- `modifiers: i32` (flags bitmask)
- `timestamp: f64`
- `native_code: u64`
- `native_extra: u64`

### Modifiers
- `None = 0`
- `Shift = 1`
- `Control = 2`
- `Alt = 4`
- `Super = 8` (cmd / win key)

### example

#### rust side

```rust
use rustapi::bindings::geode::keyboard;

let _handle = keyboard::listen(-1, 0, |data| {
    let shift_active = (data.modifiers & 1) != 0;
    let ctrl_active = (data.modifiers & 2) != 0;
    if ctrl_active && data.key == 83 { // Ctrl + S
        println!("Save shortcut triggered!");
        return true; // block propagation
    }
    false
});
```

#### cpp side

you dont need cpp side because key modifiers are checked inside the rust listener callback

