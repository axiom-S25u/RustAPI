# Open Links

open a link in the user's default browser.

### Function
- `open_link(url: &str)`

### Example
```rust
use rustapi::bindings::geode::web;

web::open_link("https://geode-sdk.org/");
```

### cpp side

you dont need cpp side because the rust open_link helper just wraps geode utils openLinkInBrowser under the hood

