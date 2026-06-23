# Task Scheduling

delays and repeating timers.

### Functions
- `delay<F>(seconds: f64, callback: F) -> u64`
- `every<F>(seconds: f64, callback: F) -> u64`
- `cancel(id: u64)`
- `run_on_main(callback: TaskCallback)`

### Example
```rust
use rustapi::bindings::task;

// delay 2.5s
let id = task::delay(2.5, |id| {
    println!("timer fired: {}", id);
});

// cancel it
task::cancel(id);
```

### cpp side

you dont need cpp side because the rust apis queue tasks using geode scheduler or dispatch loop under the hood to run on main thread safely

