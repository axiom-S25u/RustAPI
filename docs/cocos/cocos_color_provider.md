# Color Provider

use geode's color provider system.

### Functions
- `define(id: &str, r: u8, g: u8, b: u8, a: u8)`
- `override_color(id: &str, r: u8, g: u8, b: u8, a: u8)`
- `reset(id: &str)`
- `color(id: &str) -> (u8, u8, u8, u8)`
- `color3b(id: &str) -> (u8, u8, u8)`

### example

#### rust side

```rust
use rustapi::bindings::geode::color_provider;

// Define a custom theme color
color_provider::define("my-mod-accent", 255, 128, 0, 255);

// Retrieve the defined color as ccColor4B
let c = color_provider::color("my-mod-accent");
```

#### cpp side

you dont need cpp side because color definitions are registered in rust

