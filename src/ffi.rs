use std::ffi::{CString};
use std::os::raw::c_char;
use std::os::raw::c_void;

#[repr(C)]
pub struct GeodeResult {
    is_ok: bool,
    error: *mut c_char,
}

impl GeodeResult {
    pub fn ok() -> Self {
        Self {
            is_ok: true,
            error: std::ptr::null_mut(),
        }
    }

    pub fn err(msg: String) -> Self {
        let err = CString::new(msg).unwrap();
        Self {
            is_ok: false,
            error: err.into_raw(),
        }
    }
}

#[repr(C)]
pub struct GeodeMod {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GeodeNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GeodeLayer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GeodeSprite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RustKeyboardInputData {
    pub key: i32,
    pub action: i32,
    pub modifiers: i32,
    pub timestamp: f64,
    pub native_code: u64,
    pub native_extra: u64,
}

pub type RustKeyboardCallback = extern "C" fn(data: *mut RustKeyboardInputData, user_data: *mut c_void) -> bool;
pub type RustSettingCallback = extern "C" fn(key: *const c_char, json_val: *const c_char);
pub type RustAllSettingsCallback = extern "C" fn(key: *const c_char, json_val: *const c_char);
pub type RustPermissionCallback = extern "C" fn(granted: bool);
pub type RustWebRequestCallback = extern "C" fn(status: i32, headers_json: *const c_char, body: *const c_char, error: *const c_char, user_data: *mut c_void);

extern "C" {
    pub fn geode_mod_get_id() -> *mut c_char;
    pub fn geode_mod_get_name() -> *mut c_char;
    pub fn geode_mod_get_version() -> *mut c_char;
    pub fn geode_mod_get_resources_dir() -> *mut c_char;
    pub fn geode_mod_get_save_dir() -> *mut c_char;
    pub fn geode_mod_get_config_dir() -> *mut c_char;
    pub fn geode_mod_get_persistent_dir() -> *mut c_char;

    pub fn geode_mod_get_saved_value(key: *const c_char) -> *mut c_char;
    pub fn geode_mod_set_saved_value(key: *const c_char, json_val: *const c_char);

    pub fn geode_mod_get_setting_value(key: *const c_char) -> *mut c_char;
    pub fn geode_mod_has_setting(key: *const c_char) -> bool;
    pub fn geode_mod_listen_setting_changes(key: *const c_char, cb: RustSettingCallback);
    pub fn geode_mod_listen_all_setting_changes(cb: RustAllSettingsCallback);

    pub fn geode_log_info(msg: *const c_char);
    pub fn geode_log_warn(msg: *const c_char);
    pub fn geode_log_error(msg: *const c_char);
    
    pub fn geode_keybind_from_string(s: *const c_char) -> *mut c_char;
    pub fn geode_keybind_to_string(key: i32, modifiers: i32) -> *mut c_char;

    pub fn geode_clipboard_read() -> *mut c_char;
    pub fn geode_clipboard_write(text: *const c_char) -> bool;

    pub fn geode_game_exit();
    pub fn geode_game_restart();
    pub fn geode_game_launch_loader_uninstaller(delete_save: bool);

    pub fn geode_thread_get_default_name() -> *mut c_char;
    pub fn geode_thread_get_name() -> *mut c_char;
    pub fn geode_thread_set_name(name: *const c_char);

    pub fn geode_platform_get_string() -> *mut c_char;
    pub fn geode_platform_is_wine() -> bool;

    pub fn geode_random_uuid() -> *mut c_char;
    pub fn geode_random_hex_string(length: i32) -> *mut c_char;
    pub fn geode_random_alphanumeric_string(length: i32) -> *mut c_char;
    pub fn geode_random_string(length: i32, chars: *const c_char) -> *mut c_char;

    pub fn geode_ui_open_mods_list();
    pub fn geode_ui_create_default_logo() -> *mut c_void;
    pub fn geode_ui_create_server_mod_logo(mod_id: *const c_char) -> *mut c_void;

    // i mean its obv aint it?
    pub fn imgui_init_host();
    pub fn imgui_shutdown_host();
    pub fn imgui_set_visible(visible: bool);
    pub fn imgui_toggle();
    pub fn imgui_is_visible() -> bool;
    pub fn imgui_register_draw_callback(cb: extern "C" fn());

    pub fn imgui_begin_window(name: *const c_char) -> bool;
    pub fn imgui_end_window();
    pub fn imgui_begin_child(str_id: *const c_char) -> bool;
    pub fn imgui_end_child();

    pub fn imgui_text(text: *const c_char);
    pub fn imgui_text_wrapped(text: *const c_char);
    pub fn imgui_bullet_text(text: *const c_char);
    pub fn imgui_button(label: *const c_char) -> bool;
    pub fn imgui_checkbox(label: *const c_char, v: *mut bool) -> bool;
    pub fn imgui_radio_button(label: *const c_char, active: bool) -> bool;
    pub fn imgui_slider_float(label: *const c_char, v: *mut f32, v_min: f32, v_max: f32) -> bool;
    pub fn imgui_slider_int(label: *const c_char, v: *mut i32, v_min: i32, v_max: i32) -> bool;
    pub fn imgui_drag_float(label: *const c_char, v: *mut f32, v_speed: f32, v_min: f32, v_max: f32) -> bool;
    pub fn imgui_drag_int(label: *const c_char, v: *mut i32, v_speed: f32, v_min: i32, v_max: i32) -> bool;

    pub fn imgui_same_line();
    pub fn imgui_separator();
    pub fn imgui_separator_text(text: *const c_char);
    pub fn imgui_spacing();
    pub fn imgui_new_line();
    pub fn imgui_indent(w: f32);
    pub fn imgui_unindent(w: f32);
    pub fn imgui_dummy(x: f32, y: f32);
    pub fn imgui_columns(count: i32, id: *const c_char, border: bool);
    pub fn imgui_next_column();
    pub fn imgui_collapsing_header(label: *const c_char) -> bool;
    pub fn imgui_tree_node(label: *const c_char) -> bool;
    pub fn imgui_tree_pop();
    pub fn imgui_begin_group();
    pub fn imgui_end_group();

    pub fn imgui_push_id_str(id: *const c_char);
    pub fn imgui_push_id_int(id: i32);
    pub fn imgui_pop_id();

    pub fn imgui_begin_table(id: *const c_char, columns: i32) -> bool;
    pub fn imgui_end_table();
    pub fn imgui_table_next_row();
    pub fn imgui_table_next_column() -> bool;
    pub fn imgui_table_setup_column(label: *const c_char);
    pub fn imgui_table_headers_row();
    pub fn imgui_table_set_column_index(column: i32) -> bool;

    pub fn imgui_open_popup(id: *const c_char);
    pub fn imgui_begin_popup(id: *const c_char) -> bool;
    pub fn imgui_end_popup();
    pub fn imgui_begin_popup_modal(name: *const c_char) -> bool;
    pub fn imgui_close_current_popup();
    pub fn imgui_begin_tab_bar(id: *const c_char) -> bool;
    pub fn imgui_end_tab_bar();
    pub fn imgui_begin_tab_item(label: *const c_char) -> bool;
    pub fn imgui_end_tab_item();
    pub fn imgui_set_tooltip(text: *const c_char);

    pub fn imgui_begin_menu_bar() -> bool;
    pub fn imgui_end_menu_bar();
    pub fn imgui_begin_menu(label: *const c_char) -> bool;
    pub fn imgui_end_menu();
    pub fn imgui_menu_item(label: *const c_char) -> bool;

    pub fn imgui_style_colors_dark();
    pub fn imgui_style_colors_light();
    pub fn imgui_style_colors_classic();
    pub fn imgui_push_style_var_float(idx: i32, val: f32);
    pub fn imgui_push_style_var_vec2(idx: i32, x: f32, y: f32);
    pub fn imgui_pop_style_var(count: i32);
    pub fn imgui_push_style_color(idx: i32, r: f32, g: f32, b: f32, a: f32);
    pub fn imgui_pop_style_color(count: i32);

    pub fn geode_node_add_child(node: *mut GeodeNode, child: *mut GeodeNode);
    pub fn geode_node_remove_from_parent(node: *mut GeodeNode);

    // tasks
    pub fn geode_task_run_main(cb: extern "C" fn(*mut c_void), data: *mut c_void);
    pub fn geode_task_delay(seconds: f64, cb: extern "C" fn(id: u64, data: *mut c_void), data: *mut c_void) -> u64;
    pub fn geode_task_every(seconds: f64, cb: extern "C" fn(id: u64, data: *mut c_void), data: *mut c_void) -> u64;
    pub fn geode_task_cancel(id: u64);
    pub fn geode_time_now() -> f64;
    pub fn geode_time_unix() -> f64;
    // permission
    pub fn geode_permission_get_status(perm: i32) -> bool;
    pub fn geode_permission_request(perm: i32, cb: RustPermissionCallback);

    // fs
    pub fn geode_fs_read(path: *const c_char) -> *mut c_char;
    pub fn geode_fs_write(path: *const c_char, data: *const c_char) -> bool;
    pub fn geode_fs_exists(path: *const c_char) -> bool;
    pub fn geode_fs_list(path: *const c_char) -> *mut c_char;
    pub fn geode_fs_mkdir(path: *const c_char) -> bool;
    pub fn geode_fs_remove(path: *const c_char) -> bool;

    // nodes
    pub fn cocos_node_set_pos(n: *mut GeodeNode, x: f32, y: f32);
    pub fn cocos_node_get_pos(n: *mut GeodeNode, x: *mut f32, y: *mut f32);
    pub fn cocos_node_set_scale(n: *mut GeodeNode, s: f32);
    pub fn cocos_node_get_scale(n: *mut GeodeNode) -> f32;
    pub fn cocos_node_set_rot(n: *mut GeodeNode, r: f32);
    pub fn cocos_node_get_rot(n: *mut GeodeNode) -> f32;
    pub fn cocos_node_set_visible(n: *mut GeodeNode, v: bool);
    pub fn cocos_node_is_visible(n: *mut GeodeNode) -> bool;
    pub fn cocos_node_set_opacity(n: *mut GeodeNode, o: u8);
    pub fn cocos_node_get_opacity(n: *mut GeodeNode) -> u8;
    pub fn cocos_node_set_color(n: *mut GeodeNode, r: u8, g: u8, b: u8);
    pub fn cocos_node_get_color(n: *mut GeodeNode, r: *mut u8, g: *mut u8, b: *mut u8);

    pub fn cocos_node_set_id(node: *mut GeodeNode, id: *const c_char);
    pub fn cocos_node_get_id(node: *mut GeodeNode) -> *mut c_char;
    pub fn cocos_node_get_z_order(node: *mut GeodeNode) -> i32;
    pub fn cocos_node_set_z_order(node: *mut GeodeNode, z: i32);
    pub fn cocos_node_get_child_count(node: *mut GeodeNode) -> i32;
    pub fn cocos_node_set_anchor(node: *mut GeodeNode, x: f32, y: f32);
    pub fn cocos_node_get_anchor(node: *mut GeodeNode, x: *mut f32, y: *mut f32);
    pub fn cocos_node_set_content_size(node: *mut GeodeNode, w: f32, h: f32);
    pub fn cocos_node_get_content_size(node: *mut GeodeNode, w: *mut f32, h: *mut f32);
    pub fn cocos_get_win_size(w: *mut f32, h: *mut f32);
    pub fn geode_cocos_calculate_child_coverage(node: *mut GeodeNode, x: *mut f32, y: *mut f32, w: *mut f32, h: *mut f32);
    pub fn geode_cocos_calculate_node_coverage(nodes: *mut *mut GeodeNode, count: i32, x: *mut f32, y: *mut f32, w: *mut f32, h: *mut f32);
    pub fn geode_cocos_switch_to_scene(layer: *mut GeodeNode) -> *mut GeodeNode;
    pub fn geode_cocos_node_is_visible(node: *mut GeodeNode) -> bool;
    pub fn geode_cocos_get_child_by_tag_recursive(node: *mut GeodeNode, tag: i32) -> *mut GeodeNode;
    pub fn geode_cocos_is_sprite_frame_name(node: *mut GeodeNode, name: *const c_char) -> bool;
    pub fn geode_cocos_get_child_by_sprite_frame_name(node: *mut GeodeNode, name: *const c_char) -> *mut GeodeNode;
    pub fn geode_cocos_is_sprite_name(node: *mut GeodeNode, name: *const c_char) -> bool;
    pub fn geode_cocos_get_child_by_sprite_name(node: *mut GeodeNode, name: *const c_char) -> *mut GeodeNode;
    pub fn geode_cocos_get_mouse_pos(x: *mut f32, y: *mut f32);
    pub fn geode_cocos_get_label_size(text: *const c_char, font: *const c_char, kerning: f32, w: *mut f32, h: *mut f32);
    pub fn geode_cocos_file_exists_in_search_paths(filename: *const c_char) -> bool;
    pub fn geode_cocos_limit_node_size(node: *mut GeodeNode, w: f32, h: f32, def: f32, min: f32);
    pub fn geode_cocos_limit_node_width(node: *mut GeodeNode, width: f32, def: f32, min: f32);
    pub fn geode_cocos_limit_node_height(node: *mut GeodeNode, height: f32, def: f32, min: f32);


    //keyboard and webbb
    pub fn geode_keyboard_listen(key: i32, priority: i32, cb: RustKeyboardCallback, user_data: *mut c_void) -> *mut c_void;
    pub fn geode_keyboard_disconnect(handle: *mut c_void);
    
    pub fn geode_web_open_link(url: *const c_char);
    pub fn geode_web_request_new() -> *mut c_void;
    pub fn geode_web_request_free(handle: *mut c_void);
    pub fn geode_web_request_header(handle: *mut c_void, name: *const c_char, value: *const c_char);
    pub fn geode_web_request_param(handle: *mut c_void, name: *const c_char, value: *const c_char);
    pub fn geode_web_request_method(handle: *mut c_void, method: *const c_char);
    pub fn geode_web_request_timeout(handle: *mut c_void, seconds: i32);
    pub fn geode_web_request_body_string(handle: *mut c_void, data: *const c_char);
    pub fn geode_web_request_body_json(handle: *mut c_void, json: *const c_char);
    pub fn geode_web_request_send(handle: *mut c_void, method: *const c_char, url: *const c_char, cb: RustWebRequestCallback, user_data: *mut c_void);

    pub fn geode_create_quick_popup(
        title: *const c_char,
        content: *const c_char,
        btn1: *const c_char,
        btn2: *const c_char,
        cb: extern "C" fn(bool)
    );
    pub fn geode_create_alert_popup(
        title: *const c_char,
        content: *const c_char,
        btn1: *const c_char,
        btn2: *const c_char,
        cb: extern "C" fn(bool)
    );
    pub fn geode_popup_create(
        width: f32,
        height: f32,
        on_init: extern "C" fn(*mut c_void, *mut c_void),
        on_close: extern "C" fn(*mut c_void, *mut c_void),
        on_destruct: extern "C" fn(*mut c_void),
        user_data: *mut c_void
    ) -> *mut c_void;
    pub fn geode_popup_show(popup: *mut c_void);
    pub fn geode_popup_close(popup: *mut c_void);
    pub fn geode_popup_set_title(popup: *mut c_void, title: *const c_char);
    pub fn geode_popup_get_main_layer(popup: *mut c_void) -> *mut GeodeNode;
    pub fn geode_popup_get_close_btn(popup: *mut c_void) -> *mut GeodeNode;
    pub fn geode_popup_get_bg_sprite(popup: *mut c_void) -> *mut GeodeNode;
    pub fn geode_popup_get_size(popup: *mut c_void, w: *mut f32, h: *mut f32);
    pub fn geode_popup_get_button_menu(popup: *mut c_void) -> *mut GeodeNode;
    pub fn geode_popup_set_no_elasticity(popup: *mut c_void, val: bool);
    pub fn geode_popup_get_no_elasticity(popup: *mut c_void) -> bool;
    pub fn geode_popup_set_reverse_key_back(popup: *mut c_void, val: bool);
    pub fn geode_popup_get_reverse_key_back(popup: *mut c_void) -> bool;
    pub fn geode_popup_set_close_button_spr(popup: *mut c_void, sprite: *mut GeodeNode, scale: f32);



    // color provider
    pub fn geode_color_define(id: *const c_char, r: u8, g: u8, b: u8, a: u8);
    pub fn geode_color_override(id: *const c_char, r: u8, g: u8, b: u8, a: u8);
    pub fn geode_color_reset(id: *const c_char);
    pub fn geode_color_color(id: *const c_char, r: *mut u8, g: *mut u8, b: *mut u8, a: *mut u8);
    pub fn geode_color_color3b(id: *const c_char, r: *mut u8, g: *mut u8, b: *mut u8);

    // cocos color stuff
    pub fn geode_cocos_invert3b(r: u8, g: u8, b: u8, or_: *mut u8, og: *mut u8, ob: *mut u8);
    pub fn geode_cocos_lighten3b(r: u8, g: u8, b: u8, amount: i32, or_: *mut u8, og: *mut u8, ob: *mut u8);
    pub fn geode_cocos_darken3b(r: u8, g: u8, b: u8, amount: i32, or_: *mut u8, og: *mut u8, ob: *mut u8);
    pub fn geode_cocos_cc3b_from_hex(hex: *const c_char, r: *mut u8, g: *mut u8, b: *mut u8) -> bool;
    pub fn geode_cocos_cc4b_from_hex(hex: *const c_char, r: *mut u8, g: *mut u8, b: *mut u8, a: *mut u8) -> bool;
    pub fn geode_cocos_invert4b(r: u8, g: u8, b: u8, a: u8, or_: *mut u8, og: *mut u8, ob: *mut u8, oa: *mut u8);
    pub fn geode_cocos_to3b(r: u8, g: u8, b: u8, a: u8, or_: *mut u8, og: *mut u8, ob: *mut u8);
    pub fn geode_cocos_to4b(r: u8, g: u8, b: u8, alpha: u8, or_: *mut u8, og: *mut u8, ob: *mut u8, oa: *mut u8);
    pub fn geode_cocos_to4f(r: u8, g: u8, b: u8, a: u8, or_: *mut f32, og: *mut f32, ob: *mut f32, oa: *mut f32);
    pub fn geode_cocos_cc3b_to_hex(r: u8, g: u8, b: u8) -> *mut c_char;
    pub fn geode_cocos_cc4b_to_hex(r: u8, g: u8, b: u8, a: u8) -> *mut c_char;


    // version
    pub fn geode_version_parse(version_str: *const c_char) -> *mut c_char;
    pub fn geode_version_compare(a: *const c_char, b: *const c_char) -> i32;
    pub fn geode_version_matches(constraint: *const c_char, version: *const c_char) -> bool;

    // platform
    pub fn geode_platform_is_windows() -> bool;
    pub fn geode_platform_is_macos() -> bool;
    pub fn geode_platform_is_ios() -> bool;
    pub fn geode_platform_is_android() -> bool;

    pub fn geode_free_string(s: *mut c_char);
}

pub fn log_info(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { geode_log_info(c_msg.as_ptr()) };
}

pub fn log_warn(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { geode_log_warn(c_msg.as_ptr()) };
}

pub fn log_error(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { geode_log_error(c_msg.as_ptr()) };
}