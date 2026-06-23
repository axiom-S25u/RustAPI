# WebResponse

what you get back in the send callback.

### Fields
- `status: i32` — HTTP status code
- `headers: Vec<String>` — list of headers as `"Key: Value"` strings
- `body: Option<String>` — response body if request succeeded (status 2xx)
- `error: Option<String>` — error message if request failed

### example

#### rust side

```rust
use rustapi::bindings::geode::web::WebResponse;

fn handle_response(res: WebResponse) {
    if res.status == 200 {
        if let Some(body) = res.body {
            println!("Success! Body length: {}", body.len());
        }
    } else {
        if let Some(err) = res.error {
            println!("Request failed: {}", err);
        }
    }
}
```

#### cpp side

you dont need cpp side because the response data fields are accessed within the rust callback