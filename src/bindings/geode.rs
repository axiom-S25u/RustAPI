use crate::ffi;
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use serde_json::Value;
use std::os::raw::c_void;

pub struct Mod;

impl Mod {
    pub fn get() -> Self {
        Self
    }

    pub fn get_id(&self) -> String {
        unsafe {
            let ptr = ffi::geode_mod_get_id();
            if ptr.is_null() { return String::new(); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            s
        }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let ptr = ffi::geode_mod_get_name();
            if ptr.is_null() { return String::new(); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            s
        }
    }

    pub fn get_version(&self) -> String {
        unsafe {
            let ptr = ffi::geode_mod_get_version();
            if ptr.is_null() { return String::new(); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            s
        }
    }

    pub fn get_save_dir(&self) -> PathBuf {
        unsafe {
            let ptr = ffi::geode_mod_get_save_dir();
            if ptr.is_null() { return PathBuf::new(); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            PathBuf::from(s)
        }
    }

    pub fn get_resources_dir(&self) -> PathBuf {
        unsafe {
            let ptr = ffi::geode_mod_get_resources_dir();
            if ptr.is_null() { return PathBuf::new(); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            PathBuf::from(s)
        }
    }

    pub fn get_config_dir(&self) -> PathBuf {
        unsafe {
            let ptr = ffi::geode_mod_get_config_dir();
            if ptr.is_null() { return PathBuf::new(); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            PathBuf::from(s)
        }
    }

    pub fn get_persistent_dir(&self) -> PathBuf {
        unsafe {
            let ptr = ffi::geode_mod_get_persistent_dir();
            if ptr.is_null() { return PathBuf::new(); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            PathBuf::from(s)
        }
    }

    pub fn get_saved_value(&self, key: &str) -> Result<Value, String> {
        let c_key = CString::new(key).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = ffi::geode_mod_get_saved_value(c_key.as_ptr());
            if ptr.is_null() { return Ok(Value::Null); }
            let s = CStr::from_ptr(ptr).to_string_lossy();
            let val = serde_json::from_str(&s).map_err(|e| e.to_string())?;
            ffi::geode_free_string(ptr);
            Ok(val)
        }
    }

    pub fn set_saved_value(&self, key: &str, value: &Value) -> Result<(), String> {
        let c_key = CString::new(key).map_err(|e| e.to_string())?;
        let json_val = serde_json::to_string(value).map_err(|e| e.to_string())?;
        let c_val = CString::new(json_val).map_err(|e| e.to_string())?;
        unsafe {
            ffi::geode_mod_set_saved_value(c_key.as_ptr(), c_val.as_ptr());
        }
        Ok(())
    }

    pub fn get_setting_value(&self, key: &str) -> Result<Value, String> {
        let c_key = CString::new(key).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = ffi::geode_mod_get_setting_value(c_key.as_ptr());
            if ptr.is_null() { return Ok(Value::Null); }
            let s = CStr::from_ptr(ptr).to_string_lossy();
            let val = serde_json::from_str(&s).map_err(|e| e.to_string())?;
            ffi::geode_free_string(ptr);
            Ok(val)
        }
    }

    pub fn has_setting(&self, key: &str) -> bool {
        let c_key = CString::new(key).unwrap_or_default();
        unsafe { ffi::geode_mod_has_setting(c_key.as_ptr()) }
    }

    pub fn listen_for_setting_changes<F>(&self, key: &str, callback: F) -> Result<(), String>
    where
        F: FnMut(Value) + Send + 'static,
    {
        let c_key = CString::new(key).map_err(|e| e.to_string())?;
        
        struct SettingChangeCallback {
            key: String,
            cb: Box<dyn FnMut(Value) + Send>,
        }
        static mut SETTING_CALLBACKS: Option<Vec<SettingChangeCallback>> = None;
        
        unsafe {
            if SETTING_CALLBACKS.is_none() {
                SETTING_CALLBACKS = Some(Vec::new());
            }
            
            SETTING_CALLBACKS.as_mut().unwrap().push(SettingChangeCallback {
                key: key.to_string(),
                cb: Box::new(callback),
            });
            
            extern "C" fn generic_setting_cb(key_ptr: *const std::os::raw::c_char, json_val: *const std::os::raw::c_char) {
                unsafe {
                    if key_ptr.is_null() || json_val.is_null() { return; }
                    let key = CStr::from_ptr(key_ptr).to_string_lossy().into_owned();
                    let s = CStr::from_ptr(json_val).to_string_lossy();
                    if let Ok(val) = serde_json::from_str::<Value>(&s) {
                        if let Some(ref mut list) = SETTING_CALLBACKS {
                            for cb_struct in list.iter_mut() {
                                if cb_struct.key == key {
                                    (cb_struct.cb)(val.clone());
                                }
                            }
                        }
                    }
                }
            }
            
            ffi::geode_mod_listen_setting_changes(c_key.as_ptr(), generic_setting_cb);
        }
        Ok(())
    }

    pub fn listen_for_all_setting_changes<F>(&self, callback: F)
    where
        F: FnMut(String, Value) + Send + 'static,
    {
        static mut ALL_SETTING_CALLBACKS: Option<Vec<Box<dyn FnMut(String, Value) + Send>>> = None;
        
        unsafe {
            if ALL_SETTING_CALLBACKS.is_none() {
                ALL_SETTING_CALLBACKS = Some(Vec::new());
            }
            ALL_SETTING_CALLBACKS.as_mut().unwrap().push(Box::new(callback));
            
            extern "C" fn generic_all_settings_cb(key_ptr: *const std::os::raw::c_char, json_val: *const std::os::raw::c_char) {
                unsafe {
                    if key_ptr.is_null() || json_val.is_null() { return; }
                    let key = CStr::from_ptr(key_ptr).to_string_lossy().into_owned();
                    let s = CStr::from_ptr(json_val).to_string_lossy();
                    if let Ok(val) = serde_json::from_str::<Value>(&s) {
                        if let Some(ref mut list) = ALL_SETTING_CALLBACKS {
                            for cb in list.iter_mut() {
                                cb(key.clone(), val.clone());
                            }
                        }
                    }
                }
            }
            ffi::geode_mod_listen_all_setting_changes(generic_all_settings_cb);
        }
    }
}

// wraps raw cocos node pointers for rust side manipulation
pub struct Node {
    pub(crate) _ptr: *mut ffi::GeodeNode,
}

impl Node {
    pub fn from_raw(ptr: *mut ffi::GeodeNode) -> Self {
        Self { _ptr: ptr }
    }

    pub fn as_raw(&self) -> *mut ffi::GeodeNode {
        self._ptr
    }

    pub fn set_position(&self, x: f32, y: f32) {
        unsafe { ffi::cocos_node_set_pos(self._ptr, x, y) }
    }

    pub fn get_position(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { ffi::cocos_node_get_pos(self._ptr, &mut x, &mut y) }
        (x, y)
    }

    pub fn set_scale(&self, s: f32) {
        unsafe { ffi::cocos_node_set_scale(self._ptr, s) }
    }

    pub fn get_scale(&self) -> f32 {
        unsafe { ffi::cocos_node_get_scale(self._ptr) }
    }

    pub fn set_rotation(&self, r: f32) {
        unsafe { ffi::cocos_node_set_rot(self._ptr, r) }
    }

    pub fn get_rotation(&self) -> f32 {
        unsafe { ffi::cocos_node_get_rot(self._ptr) }
    }

    pub fn set_visible(&self, v: bool) {
        unsafe { ffi::cocos_node_set_visible(self._ptr, v) }
    }

    pub fn is_visible(&self) -> bool {
        unsafe { ffi::cocos_node_is_visible(self._ptr) }
    }

    pub fn set_opacity(&self, o: u8) {
        unsafe { ffi::cocos_node_set_opacity(self._ptr, o) }
    }

    pub fn get_opacity(&self) -> u8 {
        unsafe { ffi::cocos_node_get_opacity(self._ptr) }
    }

    pub fn set_color(&self, r: u8, g: u8, b: u8) {
        unsafe { ffi::cocos_node_set_color(self._ptr, r, g, b) }
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        unsafe { ffi::cocos_node_get_color(self._ptr, &mut r, &mut g, &mut b) }
        (r, g, b)
    }

    pub fn add_child(&self, child: &Node) {
        unsafe { ffi::geode_node_add_child(self._ptr, child._ptr) }
    }

    pub fn remove_from_parent(&self) {
        unsafe { ffi::geode_node_remove_from_parent(self._ptr) }
    }

    pub fn set_id(&self, id: &str) {
        let c_id = CString::new(id).unwrap_or_default();
        unsafe { ffi::cocos_node_set_id(self._ptr, c_id.as_ptr()) }
    }

    pub fn get_id(&self) -> String {
        unsafe {
            let ptr = ffi::cocos_node_get_id(self._ptr);
            if ptr.is_null() { return String::new(); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            s
        }
    }

    pub fn get_z_order(&self) -> i32 {
        unsafe { ffi::cocos_node_get_z_order(self._ptr) }
    }

    pub fn set_z_order(&self, z: i32) {
        unsafe { ffi::cocos_node_set_z_order(self._ptr, z) }
    }

    pub fn get_child_count(&self) -> i32 {
        unsafe { ffi::cocos_node_get_child_count(self._ptr) }
    }

    pub fn set_anchor(&self, x: f32, y: f32) {
        unsafe { ffi::cocos_node_set_anchor(self._ptr, x, y) }
    }

    pub fn get_anchor(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { ffi::cocos_node_get_anchor(self._ptr, &mut x, &mut y) }
        (x, y)
    }

    pub fn set_content_size(&self, w: f32, h: f32) {
        unsafe { ffi::cocos_node_set_content_size(self._ptr, w, h) }
    }

    pub fn get_content_size(&self) -> (f32, f32) {
        let mut w = 0.0;
        let mut h = 0.0;
        unsafe { ffi::cocos_node_get_content_size(self._ptr, &mut w, &mut h) }
        (w, h)
    }
}

pub fn log_info(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { ffi::geode_log_info(c_msg.as_ptr()) };
}

pub fn log_warn(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { ffi::geode_log_warn(c_msg.as_ptr()) };
}

pub fn log_error(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { ffi::geode_log_error(c_msg.as_ptr()) };
}

static mut POPUP_CALLBACK: Option<Box<dyn Fn(bool) + Send>> = None;
static mut ALERT_CALLBACK: Option<Box<dyn Fn(bool) + Send>> = None;

extern "C" fn popup_cb(b2: bool) {
    unsafe {
        if let Some(ref cb) = POPUP_CALLBACK {
            cb(b2);
        }
    }
}

extern "C" fn alert_cb(b2: bool) {
    unsafe {
        if let Some(ref cb) = ALERT_CALLBACK {
            cb(b2);
        }
    }
}

pub fn create_quick_popup<F>(title: &str, content: &str, btn1: &str, btn2: &str, callback: F)
where
    F: Fn(bool) + Send + 'static,
{
    let c_title = CString::new(title).unwrap();
    let c_content = CString::new(content).unwrap();
    let c_btn1 = CString::new(btn1).unwrap();
    let c_btn2 = CString::new(btn2).unwrap();

    unsafe {
        POPUP_CALLBACK = Some(Box::new(callback));
        ffi::geode_create_quick_popup(
            c_title.as_ptr(),
            c_content.as_ptr(),
            c_btn1.as_ptr(),
            c_btn2.as_ptr(),
            popup_cb
        );
    }
}

pub fn create_alert_popup<F>(title: &str, content: &str, btn1: &str, btn2: &str, callback: F)
where
    F: Fn(bool) + Send + 'static,
{
    let c_title = CString::new(title).unwrap();
    let c_content = CString::new(content).unwrap();
    let c_btn1 = CString::new(btn1).unwrap();
    let c_btn2 = CString::new(btn2).unwrap();

    unsafe {
        ALERT_CALLBACK = Some(Box::new(callback));
        ffi::geode_create_alert_popup(
            c_title.as_ptr(),
            c_content.as_ptr(),
            c_btn1.as_ptr(),
            c_btn2.as_ptr(),
            alert_cb
        );
    }
}

pub struct Popup {
    inner: *mut c_void,
}

impl Popup {
    pub fn get_inner(&self) -> *mut c_void {
        self.inner
    }

    pub fn set_title(&self, title: &str) {
        if let Ok(c_title) = CString::new(title) {
            unsafe {
                ffi::geode_popup_set_title(self.inner, c_title.as_ptr());
            }
        }
    }

    pub fn get_main_layer(&self) -> Option<Node> {
        let ptr = unsafe { ffi::geode_popup_get_main_layer(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Node::from_raw(ptr))
        }
    }

    pub fn get_close_btn(&self) -> Option<Node> {
        let ptr = unsafe { ffi::geode_popup_get_close_btn(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Node::from_raw(ptr))
        }
    }

    pub fn get_bg_sprite(&self) -> Option<Node> {
        let ptr = unsafe { ffi::geode_popup_get_bg_sprite(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Node::from_raw(ptr))
        }
    }

    pub fn get_size(&self) -> (f32, f32) {
        let mut w = 0.0f32;
        let mut h = 0.0f32;
        unsafe {
            ffi::geode_popup_get_size(self.inner, &mut w, &mut h);
        }
        (w, h)
    }

    pub fn get_button_menu(&self) -> Option<Node> {
        let ptr = unsafe { ffi::geode_popup_get_button_menu(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Node::from_raw(ptr))
        }
    }

    pub fn set_no_elasticity(&self, val: bool) {
        unsafe {
            ffi::geode_popup_set_no_elasticity(self.inner, val);
        }
    }

    pub fn get_no_elasticity(&self) -> bool {
        unsafe {
            ffi::geode_popup_get_no_elasticity(self.inner)
        }
    }

    pub fn set_reverse_key_back(&self, val: bool) {
        unsafe {
            ffi::geode_popup_set_reverse_key_back(self.inner, val);
        }
    }

    pub fn get_reverse_key_back(&self) -> bool {
        unsafe {
            ffi::geode_popup_get_reverse_key_back(self.inner)
        }
    }

    pub fn set_close_button_spr(&self, sprite: &Node, scale: f32) {
        unsafe {
            ffi::geode_popup_set_close_button_spr(self.inner, sprite.as_raw(), scale);
        }
    }

    pub fn show(&self) {
        unsafe {
            ffi::geode_popup_show(self.inner);
        }
    }

    pub fn close(&self) {
        unsafe {
            ffi::geode_popup_close(self.inner);
        }
    }
}

struct PopupCallbacks {
    on_init: Box<dyn FnMut(&Popup) + Send + 'static>,
    on_close: Box<dyn FnMut(&Popup) + Send + 'static>,
}

extern "C" fn raw_on_init(popup_ptr: *mut c_void, user_data: *mut c_void) {
    if !user_data.is_null() {
        let callbacks = unsafe { &mut *(user_data as *mut PopupCallbacks) };
        let popup = Popup { inner: popup_ptr };
        (callbacks.on_init)(&popup);
    }
}

extern "C" fn raw_on_close(popup_ptr: *mut c_void, user_data: *mut c_void) {
    if !user_data.is_null() {
        let callbacks = unsafe { &mut *(user_data as *mut PopupCallbacks) };
        let popup = Popup { inner: popup_ptr };
        (callbacks.on_close)(&popup);
    }
}

extern "C" fn raw_on_destruct(user_data: *mut c_void) {
    if !user_data.is_null() {
        unsafe {
            let _ = Box::from_raw(user_data as *mut PopupCallbacks);
        }
    }
}

// allocates raw popup in cpp and hooks callbacks back to rust closures
pub fn create_custom_popup<I, C>(width: f32, height: f32, on_init: I, on_close: C) -> Option<Popup>
where
    I: FnMut(&Popup) + Send + 'static,
    C: FnMut(&Popup) + Send + 'static,
{
    let callbacks = Box::into_raw(Box::new(PopupCallbacks {
        on_init: Box::new(on_init),
        on_close: Box::new(on_close),
    }));

    let ptr = unsafe {
        ffi::geode_popup_create(
            width,
            height,
            raw_on_init,
            raw_on_close,
            raw_on_destruct,
            callbacks as *mut c_void,
        )
    };

    if ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(callbacks);
        }
        None
    } else {
        Some(Popup { inner: ptr })
    }
}


pub mod fs {
    use super::*;

    pub fn read(path: &Path) -> Result<String, String> {
        let c_path = CString::new(path.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = ffi::geode_fs_read(c_path.as_ptr());
            if ptr.is_null() { return Err("failed to read file".to_string()); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            Ok(s)
        }
    }

    pub fn write(path: &Path, data: &str) -> Result<(), String> {
        let c_path = CString::new(path.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
        let c_data = CString::new(data).map_err(|e| e.to_string())?;
        if unsafe { ffi::geode_fs_write(c_path.as_ptr(), c_data.as_ptr()) } {
            Ok(())
        } else {
            Err("failed to write file".to_string())
        }
    }

    pub fn exists(path: &Path) -> bool {
        let c_path = CString::new(path.to_string_lossy().as_ref()).unwrap_or_default();
        unsafe { ffi::geode_fs_exists(c_path.as_ptr()) }
    }

    pub fn list(path: &Path) -> Result<Vec<String>, String> {
        let c_path = CString::new(path.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = ffi::geode_fs_list(c_path.as_ptr());
            if ptr.is_null() { return Err("failed to list directory".to_string()); }
            let s = CStr::from_ptr(ptr).to_string_lossy();
            let val: Value = serde_json::from_str(&s).map_err(|e| e.to_string())?;
            ffi::geode_free_string(ptr);
            if let Some(arr) = val.as_array() {
                Ok(arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            } else {
                Err("invalid directory list response".to_string())
            }
        }
    }

    pub fn mkdir(path: &Path) -> Result<(), String> {
        let c_path = CString::new(path.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
        if unsafe { ffi::geode_fs_mkdir(c_path.as_ptr()) } {
            Ok(())
        } else {
            Err("failed to create directory".to_string())
        }
    }

    pub fn remove(path: &Path) -> Result<(), String> {
        let c_path = CString::new(path.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
        if unsafe { ffi::geode_fs_remove(c_path.as_ptr()) } {
            Ok(())
        } else {
            Err("failed to remove file/directory".to_string())
        }
    }
}

pub mod cocos {
    use super::*;

    // get screen width and height from cocos director
    pub fn get_win_size() -> (f32, f32) {
        let mut w = 0.0;
        let mut h = 0.0;
        unsafe { ffi::cocos_get_win_size(&mut w, &mut h) }
        (w, h)
    }

    pub fn calculate_child_coverage(node: &Node) -> (f32, f32, f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut w = 0.0;
        let mut h = 0.0;
        unsafe {
            ffi::geode_cocos_calculate_child_coverage(node.as_raw(), &mut x, &mut y, &mut w, &mut h);
        }
        (x, y, w, h)
    }

    pub fn calculate_node_coverage(nodes: &[&Node]) -> (f32, f32, f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut w = 0.0;
        let mut h = 0.0;
        let mut raw_nodes: Vec<*mut ffi::GeodeNode> = nodes.iter().map(|n| n.as_raw()).collect();
        unsafe {
            ffi::geode_cocos_calculate_node_coverage(raw_nodes.as_mut_ptr(), raw_nodes.len() as i32, &mut x, &mut y, &mut w, &mut h);
        }
        (x, y, w, h)
    }

    pub fn switch_to_scene(layer: &Node) -> Option<Node> {
        unsafe {
            let ptr = ffi::geode_cocos_switch_to_scene(layer.as_raw());
            if ptr.is_null() {
                None
            } else {
                Some(Node::from_raw(ptr))
            }
        }
    }

    pub fn node_is_visible(node: &Node) -> bool {
        unsafe { ffi::geode_cocos_node_is_visible(node.as_raw()) }
    }

    // recursively searches children list for node with this tag
    pub fn get_child_by_tag_recursive(node: &Node, tag: i32) -> Option<Node> {
        unsafe {
            let ptr = ffi::geode_cocos_get_child_by_tag_recursive(node.as_raw(), tag);
            if ptr.is_null() {
                None
            } else {
                Some(Node::from_raw(ptr))
            }
        }
    }

    pub fn is_sprite_frame_name(node: &Node, name: &str) -> bool {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe { ffi::geode_cocos_is_sprite_frame_name(node.as_raw(), c_name.as_ptr()) }
    }

    pub fn get_child_by_sprite_frame_name(node: &Node, name: &str) -> Option<Node> {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe {
            let ptr = ffi::geode_cocos_get_child_by_sprite_frame_name(node.as_raw(), c_name.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(Node::from_raw(ptr))
            }
        }
    }

    pub fn is_sprite_name(node: &Node, name: &str) -> bool {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe { ffi::geode_cocos_is_sprite_name(node.as_raw(), c_name.as_ptr()) }
    }

    pub fn get_child_by_sprite_name(node: &Node, name: &str) -> Option<Node> {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe {
            let ptr = ffi::geode_cocos_get_child_by_sprite_name(node.as_raw(), c_name.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(Node::from_raw(ptr))
            }
        }
    }

    pub fn get_mouse_pos() -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe {
            ffi::geode_cocos_get_mouse_pos(&mut x, &mut y);
        }
        (x, y)
    }

    pub fn get_label_size(text: &str, font: &str, kerning: f32) -> (f32, f32) {
        let c_text = CString::new(text).unwrap_or_default();
        let c_font = CString::new(font).unwrap_or_default();
        let mut w = 0.0;
        let mut h = 0.0;
        unsafe {
            ffi::geode_cocos_get_label_size(c_text.as_ptr(), c_font.as_ptr(), kerning, &mut w, &mut h);
        }
        (w, h)
    }

    pub fn file_exists_in_search_paths(filename: &str) -> bool {
        let c_filename = CString::new(filename).unwrap_or_default();
        unsafe { ffi::geode_cocos_file_exists_in_search_paths(c_filename.as_ptr()) }
    }

    pub fn limit_node_size(node: &Node, w: f32, h: f32, def: f32, min: f32) {
        unsafe { ffi::geode_cocos_limit_node_size(node.as_raw(), w, h, def, min) }
    }

    pub fn limit_node_width(node: &Node, width: f32, def: f32, min: f32) {
        unsafe { ffi::geode_cocos_limit_node_width(node.as_raw(), width, def, min) }
    }

    pub fn limit_node_height(node: &Node, height: f32, def: f32, min: f32) {
        unsafe { ffi::geode_cocos_limit_node_height(node.as_raw(), height, def, min) }
    }

    pub mod color {
        use super::*;

        pub fn invert3b(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
            let mut or = 0;
            let mut og = 0;
            let mut ob = 0;
            unsafe { ffi::geode_cocos_invert3b(r, g, b, &mut or, &mut og, &mut ob) }
            (or, og, ob)
        }

        pub fn lighten3b(r: u8, g: u8, b: u8, amount: i32) -> (u8, u8, u8) {
            let mut or = 0;
            let mut og = 0;
            let mut ob = 0;
            unsafe { ffi::geode_cocos_lighten3b(r, g, b, amount, &mut or, &mut og, &mut ob) }
            (or, og, ob)
        }

        pub fn darken3b(r: u8, g: u8, b: u8, amount: i32) -> (u8, u8, u8) {
            let mut or = 0;
            let mut og = 0;
            let mut ob = 0;
            unsafe { ffi::geode_cocos_darken3b(r, g, b, amount, &mut or, &mut og, &mut ob) }
            (or, og, ob)
        }

        pub fn cc3b_from_hex(hex: &str) -> Option<(u8, u8, u8)> {
            let c_hex = CString::new(hex).unwrap_or_default();
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            if unsafe { ffi::geode_cocos_cc3b_from_hex(c_hex.as_ptr(), &mut r, &mut g, &mut b) } {
                Some((r, g, b))
            } else {
                None
            }
        }

        pub fn cc4b_from_hex(hex: &str) -> Option<(u8, u8, u8, u8)> {
            let c_hex = CString::new(hex).unwrap_or_default();
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let mut a = 0;
            if unsafe { ffi::geode_cocos_cc4b_from_hex(c_hex.as_ptr(), &mut r, &mut g, &mut b, &mut a) } {
                Some((r, g, b, a))
            } else {
                None
            }
        }

        pub fn invert4b(r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8, u8) {
            let mut or = 0;
            let mut og = 0;
            let mut ob = 0;
            let mut oa = 0;
            unsafe { ffi::geode_cocos_invert4b(r, g, b, a, &mut or, &mut og, &mut ob, &mut oa) }
            (or, og, ob, oa)
        }

        pub fn to3b(r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8) {
            let mut or = 0;
            let mut og = 0;
            let mut ob = 0;
            unsafe { ffi::geode_cocos_to3b(r, g, b, a, &mut or, &mut og, &mut ob) }
            (or, og, ob)
        }

        pub fn to4b(r: u8, g: u8, b: u8, alpha: u8) -> (u8, u8, u8, u8) {
            let mut or = 0;
            let mut og = 0;
            let mut ob = 0;
            let mut oa = 0;
            unsafe { ffi::geode_cocos_to4b(r, g, b, alpha, &mut or, &mut og, &mut ob, &mut oa) }
            (or, og, ob, oa)
        }

        pub fn to4f(r: u8, g: u8, b: u8, a: u8) -> (f32, f32, f32, f32) {
            let mut or = 0.0;
            let mut og = 0.0;
            let mut ob = 0.0;
            let mut oa = 0.0;
            unsafe { ffi::geode_cocos_to4f(r, g, b, a, &mut or, &mut og, &mut ob, &mut oa) }
            (or, og, ob, oa)
        }

        pub fn cc3b_to_hex(r: u8, g: u8, b: u8) -> String {
            unsafe {
                let ptr = ffi::geode_cocos_cc3b_to_hex(r, g, b);
                if ptr.is_null() { return String::new(); }
                let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                ffi::geode_free_string(ptr);
                s
            }
        }

        pub fn cc4b_to_hex(r: u8, g: u8, b: u8, a: u8) -> String {
            unsafe {
                let ptr = ffi::geode_cocos_cc4b_to_hex(r, g, b, a);
                if ptr.is_null() { return String::new(); }
                let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                ffi::geode_free_string(ptr);
                s
            }
        }
    }
}

pub mod color_provider {
    use super::*;

    pub fn define(id: &str, r: u8, g: u8, b: u8, a: u8) {
        let c_id = CString::new(id).unwrap_or_default();
        unsafe { ffi::geode_color_define(c_id.as_ptr(), r, g, b, a) }
    }

    pub fn override_color(id: &str, r: u8, g: u8, b: u8, a: u8) {
        let c_id = CString::new(id).unwrap_or_default();
        unsafe { ffi::geode_color_override(c_id.as_ptr(), r, g, b, a) }
    }

    pub fn reset(id: &str) {
        let c_id = CString::new(id).unwrap_or_default();
        unsafe { ffi::geode_color_reset(c_id.as_ptr()) }
    }

    pub fn color(id: &str) -> (u8, u8, u8, u8) {
        let c_id = CString::new(id).unwrap_or_default();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut a = 0;
        unsafe { ffi::geode_color_color(c_id.as_ptr(), &mut r, &mut g, &mut b, &mut a) }
        (r, g, b, a)
    }

    pub fn color3b(id: &str) -> (u8, u8, u8) {
        let c_id = CString::new(id).unwrap_or_default();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        unsafe { ffi::geode_color_color3b(c_id.as_ptr(), &mut r, &mut g, &mut b) }
        (r, g, b)
    }
}

pub mod version {
    use super::*;

    pub fn parse(version_str: &str) -> Result<String, String> {
        let c_str = CString::new(version_str).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = ffi::geode_version_parse(c_str.as_ptr());
            if ptr.is_null() { return Err("invalid version string".to_string()); }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            Ok(s)
        }
    }

    pub fn compare(a: &str, b: &str) -> i32 {
        let c_a = CString::new(a).unwrap_or_default();
        let c_b = CString::new(b).unwrap_or_default();
        unsafe { ffi::geode_version_compare(c_a.as_ptr(), c_b.as_ptr()) }
    }

    pub fn matches(constraint: &str, version: &str) -> bool {
        let c_constraint = CString::new(constraint).unwrap_or_default();
        let c_version = CString::new(version).unwrap_or_default();
        unsafe { ffi::geode_version_matches(c_constraint.as_ptr(), c_version.as_ptr()) }
    }
}

pub mod platform {
    use super::*;

    // checks compile target of geode dll at runtime to see if we are on windows mac ios or android
    pub fn is_windows() -> bool {
        unsafe { ffi::geode_platform_is_windows() }
    }

    pub fn is_macos() -> bool {
        unsafe { ffi::geode_platform_is_macos() }
    }

    pub fn is_ios() -> bool {
        unsafe { ffi::geode_platform_is_ios() }
    }

    pub fn is_android() -> bool {
        unsafe { ffi::geode_platform_is_android() }
    }

    pub fn get_string() -> String {
        unsafe {
            let ptr = ffi::geode_platform_get_string();
            if ptr.is_null() { return String::new(); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            res
        }
    }

    pub fn is_wine() -> bool {
        unsafe { ffi::geode_platform_is_wine() }
    }
}

pub mod permission {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Permission {
        RecordAudio = 1,
        Camera = 2,
        ReadExternalStorage = 3,
        WriteExternalStorage = 4,
    }

    pub fn get_status(perm: Permission) -> bool {
        unsafe { ffi::geode_permission_get_status(perm as i32) }
    }

    pub fn request<F>(perm: Permission, callback: F)
    where
        F: FnMut(bool) + Send + 'static,
    {
        static mut PERMISSION_CALLBACKS: Option<Vec<Box<dyn FnMut(bool) + Send>>> = None;
        unsafe {
            if PERMISSION_CALLBACKS.is_none() {
                PERMISSION_CALLBACKS = Some(Vec::new());
            }
            PERMISSION_CALLBACKS.as_mut().unwrap().push(Box::new(callback));
            
            extern "C" fn generic_permission_cb(granted: bool) {
                unsafe {
                    if let Some(ref mut list) = PERMISSION_CALLBACKS {
                        if !list.is_empty() {
                            let mut cb = list.remove(0);
                            cb(granted);
                        }
                    }
                }
            }
            
            ffi::geode_permission_request(perm as i32, generic_permission_cb);
        }
    }
}

pub mod keyboard {
    use super::*;

    pub struct KeyboardInputData {
        pub key: i32,
        pub action: i32,
        pub modifiers: i32,
        pub timestamp: f64,
        pub native_code: u64,
        pub native_extra: u64,
    }

    pub struct KeyboardListenerHandle {
        ptr: *mut std::ffi::c_void,
        user_data: *mut std::ffi::c_void,
    }

    impl Drop for KeyboardListenerHandle {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                unsafe { ffi::geode_keyboard_disconnect(self.ptr) };
            }
            if !self.user_data.is_null() {
                unsafe {
                    let _ = Box::from_raw(self.user_data as *mut Box<dyn FnMut(&KeyboardInputData) -> bool + Send>);
                }
            }
        }
    }

    pub fn listen<F>(key: i32, priority: i32, callback: F) -> KeyboardListenerHandle
    where
        F: FnMut(&KeyboardInputData) -> bool + Send + 'static,
    {
        let cb_box: Box<Box<dyn FnMut(&KeyboardInputData) -> bool + Send>> = Box::new(Box::new(callback));
        let user_data = Box::into_raw(cb_box) as *mut std::ffi::c_void;

        extern "C" fn generic_keyboard_cb(data_ptr: *mut ffi::RustKeyboardInputData, user_data: *mut std::ffi::c_void) -> bool {
            unsafe {
                if data_ptr.is_null() || user_data.is_null() { return false; }
                let rust_data = &*data_ptr;
                let data = KeyboardInputData {
                    key: rust_data.key,
                    action: rust_data.action,
                    modifiers: rust_data.modifiers,
                    timestamp: rust_data.timestamp,
                    native_code: rust_data.native_code,
                    native_extra: rust_data.native_extra,
                };
                let cb = &mut *(user_data as *mut Box<dyn FnMut(&KeyboardInputData) -> bool + Send>);
                cb(&data)
            }
        }

        let ptr = unsafe { ffi::geode_keyboard_listen(key, priority, generic_keyboard_cb, user_data) };
        KeyboardListenerHandle { ptr, user_data }
    }
}

pub mod web {
    use super::*;

    pub fn open_link(url: &str) {
        let c_url = CString::new(url).unwrap_or_default();
        unsafe { ffi::geode_web_open_link(c_url.as_ptr()) }
    }

    pub struct WebResponse {
        pub status: i32,
        pub headers: Vec<String>,
        pub body: Option<String>,
        pub error: Option<String>,
    }

    pub struct WebRequest {
        handle: *mut std::ffi::c_void,
    }

    impl WebRequest {
        pub fn new() -> Self {
            let handle = unsafe { ffi::geode_web_request_new() };
            Self { handle }
        }

        pub fn header(self, name: &str, value: &str) -> Self {
            let c_name = CString::new(name).unwrap_or_default();
            let c_value = CString::new(value).unwrap_or_default();
            unsafe { ffi::geode_web_request_header(self.handle, c_name.as_ptr(), c_value.as_ptr()) };
            self
        }

        pub fn param(self, name: &str, value: &str) -> Self {
            let c_name = CString::new(name).unwrap_or_default();
            let c_value = CString::new(value).unwrap_or_default();
            unsafe { ffi::geode_web_request_param(self.handle, c_name.as_ptr(), c_value.as_ptr()) };
            self
        }


        pub fn timeout(self, seconds: i32) -> Self {
            unsafe { ffi::geode_web_request_timeout(self.handle, seconds) };
            self
        }

        pub fn body_string(self, body: &str) -> Self {
            let c_body = CString::new(body).unwrap_or_default();
            unsafe { ffi::geode_web_request_body_string(self.handle, c_body.as_ptr()) };
            self
        }

        pub fn body_json(self, body: &Value) -> Self {
            let s = serde_json::to_string(body).unwrap_or_default();
            let c_body = CString::new(s).unwrap_or_default();
            unsafe { ffi::geode_web_request_body_json(self.handle, c_body.as_ptr()) };
            self
        }

        pub fn send<F>(self, method: &str, url: &str, callback: F)
        where
            F: FnOnce(Result<WebResponse, String>) + Send + 'static,
        {
            let cb_box: Box<Box<dyn FnOnce(Result<WebResponse, String>) + Send>> = Box::new(Box::new(callback));
            let user_data = Box::into_raw(cb_box) as *mut std::ffi::c_void;

            extern "C" fn generic_web_cb(
                status: i32,
                headers_json: *const std::os::raw::c_char,
                body_ptr: *const std::os::raw::c_char,
                err_ptr: *const std::os::raw::c_char,
                user_data: *mut std::ffi::c_void,
            ) {
                unsafe {
                    if user_data.is_null() { return; }
                    let cb = Box::from_raw(user_data as *mut Box<dyn FnOnce(Result<WebResponse, String>) + Send>);
                    
                    let headers = if headers_json.is_null() {
                        Vec::new()
                    } else {
                        let s = CStr::from_ptr(headers_json).to_string_lossy();
                        serde_json::from_str(&s).unwrap_or_default()
                    };

                    let body = if body_ptr.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(body_ptr).to_string_lossy().into_owned())
                    };

                    let error = if err_ptr.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(err_ptr).to_string_lossy().into_owned())
                    };

                    if let Some(err) = error.clone() {
                        cb(Err(err));
                    } else {
                        cb(Ok(WebResponse {
                            status,
                            headers,
                            body,
                            error,
                        }));
                    }
                }
            }

            let c_method = CString::new(method).unwrap_or_default();
            let c_url = CString::new(url).unwrap_or_default();
            
            let handle = self.handle;
            std::mem::forget(self);

            unsafe {
                ffi::geode_web_request_send(handle, c_method.as_ptr(), c_url.as_ptr(), generic_web_cb, user_data);
            }
        }
    }

    impl Drop for WebRequest {
        fn drop(&mut self) {
            if !self.handle.is_null() {
                unsafe { ffi::geode_web_request_free(self.handle) };
            }
        }
    }
}

pub mod json {
    use serde::Serialize;
    use serde_json::Value;

    pub fn parse(text: &str) -> Result<Value, String> {
        serde_json::from_str(text).map_err(|e| e.to_string())
    }

    pub fn dump(value: &Value, indent: Option<usize>) -> Result<String, String> {
        if let Some(indent) = indent {
            let mut s = Vec::new();
            let indent_bytes = vec![b' '; indent];
            let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
            let mut ser = serde_json::Serializer::with_formatter(&mut s, formatter);
            value.serialize(&mut ser).map_err(|e| e.to_string())?;
            Ok(String::from_utf8(s).unwrap())
        } else {
            serde_json::to_string(value).map_err(|e| e.to_string())
        }
    }
}

pub fn to_string<T: serde::Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| e.to_string())
}

pub mod base64 {
    use base64::Engine;

    pub fn encode(data: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(data)
    }

    pub fn decode(data: &str) -> Result<Vec<u8>, String> {
        base64::engine::general_purpose::STANDARD.decode(data).map_err(|e| e.to_string())
    }

    pub fn decode_string(data: &str) -> Result<String, String> {
        let bytes = base64::engine::general_purpose::STANDARD.decode(data).map_err(|e| e.to_string())?;
        String::from_utf8(bytes).map_err(|e| e.to_string())
    }
}

pub mod keybind {
    use super::*;

    pub fn from_string(s: &str) -> Result<Value, String> {
        let c_s = CString::new(s).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = ffi::geode_keybind_from_string(c_s.as_ptr());
            if ptr.is_null() { return Err("invalid keybind string".to_string()); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            let val = serde_json::from_str(&res).map_err(|e| e.to_string())?;
            Ok(val)
        }
    }

    pub fn to_string(key: i32, modifiers: i32) -> String {
        unsafe {
            let ptr = ffi::geode_keybind_to_string(key, modifiers);
            if ptr.is_null() { return String::new(); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            res
        }
    }
}

pub mod clipboard {
    use super::*;

    pub fn read() -> String {
        unsafe {
            let ptr = ffi::geode_clipboard_read();
            if ptr.is_null() { return String::new(); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            res
        }
    }

    pub fn write(text: &str) -> bool {
        let c_text = CString::new(text).unwrap_or_default();
        unsafe { ffi::geode_clipboard_write(c_text.as_ptr()) }
    }
}

pub mod game {
    use super::*;

    pub fn exit() {
        unsafe { ffi::geode_game_exit() }
    }

    pub fn restart() {
        unsafe { ffi::geode_game_restart() }
    }

    pub fn launch_loader_uninstaller(delete_save: bool) {
        unsafe { ffi::geode_game_launch_loader_uninstaller(delete_save) }
    }
}

pub mod thread {
    use super::*;

    pub fn get_default_name() -> String {
        unsafe {
            let ptr = ffi::geode_thread_get_default_name();
            if ptr.is_null() { return String::new(); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            res
        }
    }

    pub fn get_name() -> String {
        unsafe {
            let ptr = ffi::geode_thread_get_name();
            if ptr.is_null() { return String::new(); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            res
        }
    }

    pub fn set_name(name: &str) {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe { ffi::geode_thread_set_name(c_name.as_ptr()) }
    }
}


pub mod random {
    use super::*;

    pub fn generate_uuid() -> String {
        unsafe {
            let ptr = ffi::geode_random_uuid();
            if ptr.is_null() { return String::new(); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            res
        }
    }

    pub fn generate_hex_string(length: i32) -> String {
        unsafe {
            let ptr = ffi::geode_random_hex_string(length);
            if ptr.is_null() { return String::new(); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            res
        }
    }

    pub fn generate_alphanumeric_string(length: i32) -> String {
        unsafe {
            let ptr = ffi::geode_random_alphanumeric_string(length);
            if ptr.is_null() { return String::new(); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            res
        }
    }

    pub fn generate_string(length: i32, chars: &str) -> String {
        let c_chars = CString::new(chars).unwrap_or_default();
        unsafe {
            let ptr = ffi::geode_random_string(length, c_chars.as_ptr());
            if ptr.is_null() { return String::new(); }
            let res = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::geode_free_string(ptr);
            res
        }
    }
}

pub mod ui {
    use super::*;

    pub fn open_mods_list() {
        unsafe { ffi::geode_ui_open_mods_list() }
    }

    pub fn create_default_logo() -> Option<Node> {
        unsafe {
            let ptr = ffi::geode_ui_create_default_logo();
            if ptr.is_null() { return None; }
            Some(Node { _ptr: ptr as *mut _ })
        }
    }

    pub fn create_server_mod_logo(mod_id: &str) -> Option<Node> {
        let c_id = CString::new(mod_id).unwrap_or_default();
        unsafe {
            let ptr = ffi::geode_ui_create_server_mod_logo(c_id.as_ptr());
            if ptr.is_null() { return None; }
            Some(Node { _ptr: ptr as *mut _ })
        }
    }
}
