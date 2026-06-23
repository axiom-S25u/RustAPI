# Keyboard Input

listen to keyboard input

### example

#### rust side

```rust
use rustapi::bindings::geode::keyboard;

let _handle = keyboard::listen(32, 0, |data| {
    println!("Space key pressed!");
    false
});
```

#### cpp side

you dont need cpp side because keyboard listeners are handled in rust

