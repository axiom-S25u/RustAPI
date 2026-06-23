# RustAPI Docs

here is the list of topics:

- [Getting Started](getting-started/intro.md)
  - [Intro](getting-started/intro.md)
  - [First Mod](getting-started/first_mod.md)
  - [Project Structure](getting-started/project_structure.md)
  - [CI Workflow](getting-started/ci.md)
- [Mod Info & Settings](mod/mod_intro.md)
  - [Overview](mod/mod_intro.md)
  - [Mod Metadata](mod/mod_info.md)
  - [Settings & Saved Values](mod/mod_settings.md)
  - [Logging](mod/mod_logging.md)
- [Filesystem](filesystem/fs_intro.md)
  - [Overview](filesystem/fs_intro.md)
  - [Read/Write](filesystem/fs_read_write.md)
  - [Folder Operations](filesystem/fs_operations.md)
- [Web](web/web_intro.md)
  - [Overview](web/web_intro.md)
  - [Simple Fetch](web/web_fetch.md)
  - [Request Builder](web/web_request_builder.md)
  - [Response Struct](web/web_response.md)
  - [Open Links](web/web_open_link.md)
- [Keyboard](keyboard/keyboard_intro.md)
  - [Overview](keyboard/keyboard_intro.md)
  - [Listen to Keys](keyboard/keyboard_listen.md)
  - [Modifiers & Key Codes](keyboard/keyboard_modifiers.md)
- [ImGui](imgui/imgui_intro.md)
  - [Overview](imgui/imgui_intro.md)
  - [Windows](imgui/imgui_windows.md)
  - [Widgets](imgui/imgui_widgets.md)
  - [Layout](imgui/imgui_layout.md)
  - [Tables](imgui/imgui_tables.md)
  - [Popups](imgui/imgui_popups.md)
  - [Style & Colors](imgui/imgui_style.md)
- [Cocos2d-x](cocos/cocos_intro.md)
  - [Overview](cocos/cocos_intro.md)
  - [CCNode Properties](cocos/cocos_nodes.md)
  - [Color Utilities](cocos/cocos_color_utils.md)
  - [Color Provider](cocos/cocos_color_provider.md)
- [Tasks & Scheduling](task/task_intro.md)
  - [Overview](task/task_intro.md)
  - [Timers](task/task_scheduling.md)
  - [Time Utilities](task/task_time.md)
- [WebSockets](websocket/ws_intro.md)
  - [Overview](websocket/ws_intro.md)
  - [Client](websocket/ws_client.md)
- [Utilities](utils/json.md)
  - [JSON](utils/json.md)
  - [Base64](utils/base64.md)
  - [Versions](utils/version.md)
  - [Permissions](utils/permission.md)
  - [Keybinds](utils/keybind.md)
  - [Quick Popups](utils/popup.md)
  - [Platform Detection](utils/platform.md)
  - [Game](utils/game.md)
  - [Thread](utils/thread.md)
  - [Random](utils/random.md)
  - [UI Helpers](utils/ui.md)

## quick example

here is a quick example of a mod with both cpp and rust working together

### cpp side

you need a basic cpp loader to load the rust code

```cpp
#include <Geode/Geode.hpp>
#include <RustAPI.hpp>

extern "C" {
    void my_mod_init();
}

$on_mod(Loaded) {
    my_mod_init();
}
```

### rust side

now write the actual logic in rust

```rust
use rustapi::bindings::geode::log_info;

#[no_mangle]
pub extern "C" fn my_mod_init() {
    log_info("RustAPI is working!");
}
```

