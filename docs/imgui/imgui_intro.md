# ImGui Overview

rustapi has bindings for dear imgui. you can draw custom windows, menus, and dev menus overlaying the game.

### example

#### rust side

```rust
use rustapi::bindings::imgui;

if imgui::begin_window("Debug Panel") {
    imgui::text("Hello, ImGui from Rust!");
    imgui::end_window();
}
```

#### cpp side

you dont need cpp side because imgui windows are drawn directly using rust imgui bindings

