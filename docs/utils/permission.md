# OS Permissions

check or request permissions (android / ios).

### Functions
- `get_status(perm: Permission) -> bool`
- `request<F>(perm: Permission, callback: F)`

### example

#### rust side

```rust
use rustapi::bindings::geode::permission::{self, Permission};

if !permission::get_status(Permission::RecordAudio) {
    permission::request(Permission::RecordAudio, |granted| {
        println!("Audio record permission granted: {}", granted);
    });
}
```

#### cpp side

you dont need cpp side because permissions are handled on the rust side

