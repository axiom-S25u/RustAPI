# Web Overview

requests use geode web request stuff

### example

#### rust side

```rust
use rustapi::bindings::geode::web::WebRequest;

WebRequest::new().send("GET", "https://api.github.com", |res| {
    println!("Status: {}", res.status);
});
```

#### cpp side

you dont need cpp side because web requests run asynchronously in rust using the wrapper