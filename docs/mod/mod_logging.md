# Logging

logging to the logs of the logging folder (im going insane)

### Functions
- `log_info(msg: &str)`
- `log_warn(msg: &str)`
- `log_error(msg: &str)`

### example
```rust
use rustapi::bindings::geode::{log_info, log_warn};

log_info("mod hooks set up");
log_warn("something went wrong but we skipped it");
```

### cpp side

you dont need cpp side here because the rust log helpers just redirect to geode logging system under the hood

