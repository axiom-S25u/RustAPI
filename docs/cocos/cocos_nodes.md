# Cocos Nodes

Inspect and modify Cocos2d-x CCNode elements.

### Node Methods
These methods are available on any `Node` instance:
- `set_position(&self, x: f32, y: f32)`
- `get_position(&self) -> (f32, f32)`
- `set_scale(&self, s: f32)`
- `get_scale(&self) -> f32`
- `set_rotation(&self, r: f32)`
- `get_rotation(&self) -> f32`
- `set_visible(&self, v: bool)`
- `is_visible(&self) -> bool`
- `set_opacity(&self, o: u8)`
- `get_opacity(&self) -> u8`
- `set_color(&self, r: u8, g: u8, b: u8)`
- `get_color(&self) -> (u8, u8, u8)`
- `add_child(&self, child: &Node)`
- `remove_from_parent(&self)`
- `set_id(&self, id: &str)`
- `get_id(&self) -> String`
- `get_z_order(&self) -> i32`
- `set_z_order(&self, z: i32)`
- `get_child_count(&self) -> i32`
- `set_anchor(&self, x: f32, y: f32)`
- `get_anchor(&self) -> (f32, f32)`
- `set_content_size(&self, w: f32, h: f32)`
- `get_content_size(&self) -> (f32, f32)`

### Cocos Utility Functions
These utility functions are located under the `cocos` module namespace:
- `cocos::get_win_size() -> (f32, f32)`: Returns the game window size in cocos coordinates.
- `cocos::calculate_child_coverage(node: &Node) -> (f32, f32, f32, f32)`: Calculates the bounding box that covers all children of a node, returning `(x, y, w, h)`.
- `cocos::calculate_node_coverage(nodes: &[&Node]) -> (f32, f32, f32, f32)`: Calculates the bounding box covering a list of nodes, returning `(x, y, w, h)`.
- `cocos::switch_to_scene(layer: &Node) -> Option<Node>`: Creates a new scene containing the layer and runs it.
- `cocos::node_is_visible(node: &Node) -> bool`: Checks recursively if a node and all of its ancestors are visible.
- `cocos::get_child_by_tag_recursive(node: &Node, tag: i32) -> Option<Node>`: Traverses the node tree recursively to find a child node with the specified tag.
- `cocos::is_sprite_frame_name(node: &Node, name: &str) -> bool`: Returns true if the node is a sprite or button that uses a sprite frame with the given name.
- `cocos::get_child_by_sprite_frame_name(node: &Node, name: &str) -> Option<Node>`: Recursively searches children for one matching a sprite frame name.
- `cocos::is_sprite_name(node: &Node, name: &str) -> bool`: Returns true if the node is a sprite or button that uses a sprite file with the given name.
- `cocos::get_child_by_sprite_name(node: &Node, name: &str) -> Option<Node>`: Recursively searches children for one matching a sprite file name.
- `cocos::get_mouse_pos() -> (f32, f32)`: Returns current cursor position in cocos coordinates.
- `cocos::get_label_size(text: &str, font: &str, kerning: f32) -> (f32, f32)`: Returns the width and height of a text string rendered with a specific font and kerning.
- `cocos::file_exists_in_search_paths(filename: &str) -> bool`: Checks if a file exists in the Cocos search paths.
- `cocos::limit_node_size(node: &Node, w: f32, h: f32, def: f32, min: f32)`: Scales the node down if its size exceeds the target bounds.
- `cocos::limit_node_width(node: &Node, width: f32, def: f32, min: f32)`: Scales the node down if its width exceeds the target bounds.
- `cocos::limit_node_height(node: &Node, height: f32, def: f32, min: f32)`: Scales the node down if its height exceeds the target bounds.

### Example - Adjusting Node properties
```rust
use rustapi::bindings::geode::Node;

fn tweak_node(node: &Node) {
    node.set_position(100.0, 150.0);
    node.set_scale(1.5);
    node.set_color(255, 0, 0); // red tint
    node.set_opacity(128); // semi-transparent
}
```

### Example - Node Coverage & Layout
```rust
use rustapi::bindings::geode::cocos;
use rustapi::bindings::geode::Node;

fn layout_node(parent: &Node, element1: &Node, element2: &Node) {
    // get bounding box of both elements
    let (x, y, w, h) = cocos::calculate_node_coverage(&[element1, element2]);
    
    // reposition parent based on child coverage size
    parent.set_content_size(w + 20.0, h + 20.0);
}
```

### Example - Recurse Child Search
```rust
use rustapi::bindings::geode::cocos;
use rustapi::bindings::geode::Node;

fn find_ui_components(menu: &Node) {
    // search recursively for close button by tag
    if let Some(close_btn) = cocos::get_child_by_tag_recursive(menu, 1) {
        close_btn.set_scale(1.2);
    }
    
    // search recursively for a specific button sprite frame
    if let Some(ok_btn) = cocos::get_child_by_sprite_frame_name(menu, "GJ_okBtn_001.png") {
        ok_btn.set_visible(true);
    }
}
```

### Example - Node Sizing Constraints
```rust
use rustapi::bindings::geode::cocos;
use rustapi::bindings::geode::Node;

fn setup_text_label(label: &Node) {
    // limit label width to 100.0, default scale 1.0, minimum scale 0.5
    cocos::limit_node_width(label, 100.0, 1.0, 0.5);
}
```
writing docs makes me want to end my life

### example - passing node from cpp to rust

if you hook a class in cpp (like MenuLayer::init) and want to manipulate a node in rust:

#### cpp side

```cpp
#include <Geode/Geode.hpp>
#include <RustAPI.hpp>

extern "C" {
    void handle_my_node(cocos2d::CCNode* node);
}

// within some hook (e.g. MenuLayer::init)
handle_my_node(this);
```

#### rust side

```rust
use rustapi::bindings::geode::Node;

#[no_mangle]
pub extern "C" fn handle_my_node(raw_node: *mut std::ffi::c_void) {
    let node = Node::from_raw(raw_node as *mut _);
    node.set_position(100.0, 200.0);
    node.set_scale(1.5);
}
```