# Mod Object

you can get the current mod singleton like this:

```rust
use rustapi::bindings::geode::Mod;

let my_mod = Mod::get();
```

### cpp side

you dont need cpp side here because the rust mod::get function gets the current mod instance under the hood using the ffi bindings

