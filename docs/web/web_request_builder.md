# WebRequest Builder

configure headers, timeouts, and body before sending.

### Methods
- `new() -> WebRequest`
- `header(self, name: &str, value: &str) -> Self`
- `param(self, name: &str, value: &str) -> Self`
- `timeout(self, seconds: i32) -> Self`
- `body_string(self, body: &str) -> Self`
- `body_json(self, body: &serde_json::Value) -> Self`
- `send<F>(self, method: &str, url: &str, callback: F)`


### Example
```rust
use rustapi::bindings::geode::web::WebRequest;
use serde_json::json;

WebRequest::new()
    .header("Content-Type", "application/json")
    .timeout(10)
    .body_json(&json!({"name": "axiom"}))
    .send("POST", "https://httpbin.org/post", |res| {
        if let Some(body) = res.body {
            log::info!("got: {}", body);
        }
    });
```

### cpp side

you dont need cpp side because all the request parameters are set up on the rust side