# ImGui Tables

grid tables.

### Functions
- `begin_table(id: &str, columns: i32) -> bool`
- `end_table()`
- `table_next_row()`
- `table_next_column() -> bool`
- `table_setup_column(label: &str)`
- `table_headers_row()`
- `table_set_column_index(column: i32) -> bool`

### Example
```rust
use rustapi::bindings::imgui;

if imgui::begin_table("my_table", 2) {
    imgui::table_setup_column("Player");
    imgui::table_setup_column("Score");
    imgui::table_headers_row();

    imgui::table_next_row();
    imgui::table_next_column();
    imgui::text("Axiom");
    imgui::table_next_column();
    imgui::text("420");

    imgui::end_table();
}
```

### cpp side

you dont need cpp side because the rust table api wraps the imgui table calls under the hood

