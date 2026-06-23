# Popups

spawn alert dialogs or custom popups.

### Functions
- `create_quick_popup<F>(title: &str, content: &str, btn1: &str, btn2: &str, callback: F)`
- `create_alert_popup<F>(title: &str, content: &str, btn1: &str, btn2: &str, callback: F)`
- `create_custom_popup<I, C>(width: f32, height: f32, on_init: I, on_close: C) -> Option<Popup>`

### Popup Struct Methods
if you use `create_custom_popup`, you get back a `Popup` instance with these methods:
- `set_title(&self, title: &str)`: sets the title of the popup.
- `get_main_layer(&self) -> Option<Node>`: gets the main content layer.
- `get_close_btn(&self) -> Option<Node>`: gets the default close button.
- `get_bg_sprite(&self) -> Option<Node>`: gets the popup background sprite node.
- `get_button_menu(&self) -> Option<Node>`: gets the default button menu.
- `get_size(&self) -> (f32, f32)`: gets the width and height of the popup.
- `set_no_elasticity(&self, val: bool)` / `get_no_elasticity(&self) -> bool`: gets or sets if the opening animation should skip elasticity transition.
- `set_reverse_key_back(&self, val: bool)` / `get_reverse_key_back(&self) -> bool`: toggles escape/back-key closing behavior.
- `set_close_button_spr(&self, sprite: &Node, scale: f32)`: sets a custom close button sprite.
- `show(&self)`: shows the popup on screen.
- `close(&self)`: closes and cleans up the popup.

### Example - Quick Popup
```rust
use rustapi::bindings::geode::create_quick_popup;

create_quick_popup("hey", "this is a quick popup", "ok", "cancel", |clicked_cancel| {
    if clicked_cancel {
        println!("clicked cancel");
    } else {
        println!("clicked ok");
    }
});
```

### Example - Alert Popup
```rust
use rustapi::bindings::geode::create_alert_popup;

create_alert_popup("alert", "this is a normal alert popup", "close", "action", |clicked_action| {
    if clicked_action {
        println!("clicked action");
    } else {
        println!("clicked close");
    }
});
```

### Example - Custom Popup
this lets you build a custom geode popup subclass. it calls `init(width, height)` under the hood. you get a callback to add custom layout elements to the main layer.

```rust
use rustapi::bindings::geode::{create_custom_popup, Popup};

if let Some(popup) = create_custom_popup(
    240.0, 
    160.0, 
    |popup: &Popup| {
        // on_init: set title and configure layout
        popup.set_title("my custom popup");
        popup.set_no_elasticity(true); // skip bounce animation
        
        if let Some(main_layer) = popup.get_main_layer() {
            // you can add child nodes to main_layer here
            println!("popup main layer content size: {:?}", main_layer.get_content_size());
        }
        
        if let Some(menu) = popup.get_button_menu() {
            // you can add buttons to the default menu here
        }
    }, 
    |popup: &Popup| {
        // on_close callback
        println!("popup closed");
    }
) {
    popup.show();
}
```

### cpp side

you dont need cpp side because the rust popup apis wrap the geode cpp popup subclass and methods mapping callbacks dynamically

