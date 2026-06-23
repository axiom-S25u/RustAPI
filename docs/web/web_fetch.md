# Simple Web Request

```rust
use rustapi::bindings::geode::web::WebRequest;

WebRequest::new()
    .send("GET", "https://api.github.com/repos/geode-sdk/geode", |res| {
        log::info!("status: {}", res.status);
        if let Some(body) = res.body {
            log::info!("body: {}", body);
        }
        if let Some(err) = res.error {
            log::error!("failed: {}", err);
        }
    });
```

### cpp side

you dont need cpp side because the rust webrequest dispatches everything asynchronously and hits the callback on main thread
