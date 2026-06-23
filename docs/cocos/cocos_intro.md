# Cocos2d-x Overview

GD runs on cocos2d-x. rustapi has bindings to change `CCNode` objects directly (move buttons, hide things, tweak layouts, etc.).

### example

#### rust side

```rust
use rustapi::bindings::geode::cocos;

let (win_w, win_h) = cocos::get_win_size();
println!("Screen Size: {}x{}", win_w, win_h);
```

#### cpp side

you dont need cpp side because node actions are run directly in rust

