# ImGui Widgets

interactive elements.

### Functions
- `text(text: &str)`
- `text_wrapped(text: &str)`
- `bullet_text(text: &str)`
- `button(label: &str) -> bool` (true when clicked)
- `checkbox(label: &str, v: &mut bool) -> bool`
- `radio_button(label: &str, active: bool) -> bool`
- `slider_float(label: &str, v: &mut f32, min: f32, max: f32) -> bool`
- `slider_int(label: &str, v: &mut i32, min: i32, max: i32) -> bool`
- `drag_float(label: &str, v: &mut f32, speed: f32, min: f32, max: f32) -> bool`
- `drag_int(label: &str, v: &mut i32, speed: f32, min: i32, max: i32) -> bool`

### example

#### rust side

```rust
use rustapi::bindings::imgui;

static mut SHOW_DEBUG: bool = false;
static mut SCALE_FACTOR: f32 = 1.0;

unsafe {
    imgui::text("Widget Demo");
    if imgui::button("Reset Scale") {
        SCALE_FACTOR = 1.0;
    }
    imgui::checkbox("Show Debug Logs", &mut SHOW_DEBUG);
    imgui::slider_float("Scale", &mut SCALE_FACTOR, 0.5, 2.0);
}
```

#### cpp side

you dont need cpp side because these widgets call dear imgui ffi directly

