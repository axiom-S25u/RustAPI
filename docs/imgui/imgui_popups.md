# ImGui Popups

popups, menus and modals.

### Functions
- `open_popup(id: &str)`
- `begin_popup(id: &str) -> bool`
- `end_popup()`
- `begin_popup_modal(name: &str) -> bool`
- `close_current_popup()`
- `begin_menu_bar() -> bool`
- `end_menu_bar()`
- `begin_menu(label: &str) -> bool`
- `end_menu()`
- `menu_item(label: &str) -> bool`

### example

#### rust side

```rust
use rustapi::bindings::imgui;

if imgui::button("Open Options") {
    imgui::open_popup("options_popup");
}

if imgui::begin_popup("options_popup") {
    imgui::text("Popup Settings");
    if imgui::button("Close") {
        imgui::close_current_popup();
    }
    imgui::end_popup();
}
```

#### cpp side

you dont need cpp side because modals and popups are rendered in rust

