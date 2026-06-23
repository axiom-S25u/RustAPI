# Tasks & Time

schedule events to run on the main thread safely

### example

#### rust side

```rust
use rustapi::bindings::task;

task::delay(1.0, |_id| {
    println!("Fired after 1 second!");
});
```

#### cpp side

you dont need cpp side because scheduling timers is done in rust