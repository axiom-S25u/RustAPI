# ImGui Layout

formatting layout.

### Functions
- `same_line()` (keeps next widget on same row)
- `separator()` (draws line)
- `separator_text(text: &str)`
- `spacing()`
- `new_line()`
- `indent(w: f32)`
- `unindent(w: f32)`
- `columns(count: i32, id: &str, border: bool)`
- `next_column()`
- `begin_group()`
- `end_group()`

### example

#### rust side

```rust
use rustapi::bindings::imgui;

imgui::separator_text("Layout Columns");
imgui::columns(2, "layout_demo_cols", true);

imgui::text("Column 1 Content");
imgui::next_column();

imgui::text("Column 2 Content");
imgui::columns(1, "", false); // Reset columns
```

#### cpp side

you dont need cpp side because layout formatting is done directly in rust

