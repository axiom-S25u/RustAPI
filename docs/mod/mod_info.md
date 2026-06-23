# Mod Info & Folders

get basic info or paths.

### Metadata
- `get_id(&self) -> String`
- `get_name(&self) -> String`
- `get_version(&self) -> String`

### Paths
these return a PathBuf:
- `get_save_dir(&self) -> PathBuf`
- `get_config_dir(&self) -> PathBuf`
- `get_resources_dir(&self) -> PathBuf`
- `get_persistent_dir(&self) -> PathBuf`

### example

#### rust side

```rust
use rustapi::bindings::geode::Mod;

let my_mod = Mod::get();
let id = my_mod.get_id();
let name = my_mod.get_name();
let save_dir = my_mod.get_save_dir();
```

#### cpp side

you dont need cpp side because these get info directly from the rust mod wrapper

