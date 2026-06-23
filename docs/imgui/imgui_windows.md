# ImGui Windows

draw windows and child panels.

### Functions
- `begin_window(name: &str) -> bool`: returns true if visible.
- `end_window()`
- `begin_child(str_id: &str) -> bool`
- `end_child()`

### Example
```rust
use rustapi::bindings::imgui;

if imgui::begin_window("My Panel") {
    imgui::text("hello there");
    imgui::end_window();
}
```

### cpp side

you dont need cpp side because these map to dear imgui ffi calls under the hood drawing inside the geode imgui context

