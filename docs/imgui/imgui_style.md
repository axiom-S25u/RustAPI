# ImGui Styling & Themes

themes and style customization.

### Functions
- `style_colors_dark()`
- `style_colors_light()`
- `style_colors_classic()`
- `push_style_var_float(idx: i32, val: f32)`
- `push_style_var_vec2(idx: i32, x: f32, y: f32)`
- `pop_style_var(count: i32)`
- `push_style_color(idx: i32, r: f32, g: f32, b: f32, a: f32)`
- `pop_style_color(count: i32)`

### example

#### rust side

```rust
use rustapi::bindings::imgui;

// push a red color for text (ImGuiCol_Text is index 0)
imgui::push_style_color(0, 1.0, 0.0, 0.0, 1.0);
imgui::text("This text is bright red!");
imgui::pop_style_color(1);
```

#### cpp side

you dont need cpp side because style vars are pushed and popped in the rust draw loop

