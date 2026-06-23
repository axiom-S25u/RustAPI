#pragma once

#include <Geode/Geode.hpp>
#include <cocos2d.h>

#if defined(GEODE_IS_WINDOWS)
    #ifdef RUSTAPI_EXPORTING
        #define RUSTAPI_DLL __declspec(dllexport)
    #else
        #define RUSTAPI_DLL __declspec(dllimport)
    #endif
#elif defined(GEODE_IS_MACOS) || defined(GEODE_IS_IOS) || defined(GEODE_IS_ANDROID)
    #define RUSTAPI_DLL __attribute__((visibility("default")))
#else
    #define RUSTAPI_DLL
#endif

extern "C" {
    // logging
    RUSTAPI_DLL void geode_log_info(const char* msg);
    RUSTAPI_DLL void geode_log_warn(const char* msg);
    RUSTAPI_DLL void geode_log_error(const char* msg);
    RUSTAPI_DLL void geode_log_debug(const char* msg);

    // popup
    RUSTAPI_DLL void geode_create_quick_popup(
        const char* title,
        const char* content,
        const char* btn1,
        const char* btn2,
        void (*callback)(bool)
    );
    RUSTAPI_DLL void geode_create_alert_popup(
        const char* title,
        const char* content,
        const char* btn1,
        const char* btn2,
        void (*callback)(bool)
    );
    RUSTAPI_DLL void* geode_popup_create(
        float width,
        float height,
        void (*on_init)(void* popup, void* user_data),
        void (*on_close)(void* popup, void* user_data),
        void (*on_destruct)(void* user_data),
        void* user_data
    );
    RUSTAPI_DLL void geode_popup_show(void* popup);
    RUSTAPI_DLL void geode_popup_close(void* popup);
    RUSTAPI_DLL void geode_popup_set_title(void* popup, const char* title);
    RUSTAPI_DLL void* geode_popup_get_main_layer(void* popup);
    RUSTAPI_DLL void* geode_popup_get_close_btn(void* popup);
    RUSTAPI_DLL void* geode_popup_get_bg_sprite(void* popup);
    RUSTAPI_DLL void geode_popup_get_size(void* popup, float* w, float* h);
    RUSTAPI_DLL void* geode_popup_get_button_menu(void* popup);
    RUSTAPI_DLL void geode_popup_set_no_elasticity(void* popup, bool val);
    RUSTAPI_DLL bool geode_popup_get_no_elasticity(void* popup);
    RUSTAPI_DLL void geode_popup_set_reverse_key_back(void* popup, bool val);
    RUSTAPI_DLL bool geode_popup_get_reverse_key_back(void* popup);
    RUSTAPI_DLL void geode_popup_set_close_button_spr(void* popup, void* sprite, float scale);

    // node utils
    RUSTAPI_DLL void geode_node_set_position(void* node, float x, float y);
    RUSTAPI_DLL void geode_node_get_position(void* node, float* x, float* y);
    RUSTAPI_DLL void geode_node_set_scale(void* node, float scale);
    RUSTAPI_DLL float geode_node_get_scale(void* node);
    RUSTAPI_DLL void geode_node_set_visible(void* node, bool visible);
    RUSTAPI_DLL bool geode_node_get_visible(void* node);
    RUSTAPI_DLL void* geode_get_current_scene();

    // keyboardddd
    struct RustKeyboardInputData {
        int key;
        int action;
        int modifiers;
        double timestamp;
        uint64_t native_code;
        uint64_t native_extra;
    };
    typedef bool (*RustKeyboardCallback)(RustKeyboardInputData* data, void* user_data);
    RUSTAPI_DLL void* geode_keyboard_listen(int key, int priority, RustKeyboardCallback cb, void* user_data);
    RUSTAPI_DLL void geode_keyboard_disconnect(void* handle);

    // mod info and settings
    RUSTAPI_DLL char* geode_mod_get_id();
    RUSTAPI_DLL char* geode_mod_get_name();
    RUSTAPI_DLL char* geode_mod_get_version();
    RUSTAPI_DLL char* geode_mod_get_resources_dir();
    RUSTAPI_DLL char* geode_mod_get_save_dir();
    RUSTAPI_DLL char* geode_mod_get_config_dir();
    RUSTAPI_DLL char* geode_mod_get_persistent_dir();

    // saved values
    RUSTAPI_DLL char* geode_mod_get_saved_value(const char* key);
    RUSTAPI_DLL void geode_mod_set_saved_value(const char* key, const char* json_val);

    // mod settings
    RUSTAPI_DLL char* geode_mod_get_setting_value(const char* key);
    RUSTAPI_DLL bool geode_mod_has_setting(const char* key);
    typedef void (*RustSettingCallback)(const char* key, const char* json_val);
    RUSTAPI_DLL void geode_mod_listen_setting_changes(const char* key, RustSettingCallback cb);
    typedef void (*RustAllSettingsCallback)(const char* key, const char* json_val);
    RUSTAPI_DLL void geode_mod_listen_all_setting_changes(RustAllSettingsCallback cb);

    // filesystem
    RUSTAPI_DLL char* geode_fs_read(const char* path);
    RUSTAPI_DLL bool geode_fs_write(const char* path, const char* data);
    RUSTAPI_DLL bool geode_fs_exists(const char* path);
    RUSTAPI_DLL char* geode_fs_list(const char* path);
    RUSTAPI_DLL bool geode_fs_mkdir(const char* path);
    RUSTAPI_DLL bool geode_fs_remove(const char* path);

    // json
    RUSTAPI_DLL char* geode_json_parse(const char* json_str);
    RUSTAPI_DLL char* geode_json_dump(const char* json_val, int indent);

    // base64
    RUSTAPI_DLL char* geode_base64_encode(const char* data, int variant);
    RUSTAPI_DLL char* geode_base64_decode(const char* data, int variant);
    RUSTAPI_DLL char* geode_base64_decode_string(const char* data, int variant);

    // version info
    RUSTAPI_DLL char* geode_version_parse(const char* version_str);
    RUSTAPI_DLL int geode_version_compare(const char* a, const char* b);
    RUSTAPI_DLL bool geode_version_matches(const char* constraint, const char* version);

    // color provider
    RUSTAPI_DLL void geode_color_define(const char* id, uint8_t r, uint8_t g, uint8_t b, uint8_t a);
    RUSTAPI_DLL void geode_color_override(const char* id, uint8_t r, uint8_t g, uint8_t b, uint8_t a);
    RUSTAPI_DLL void geode_color_reset(const char* id);
    RUSTAPI_DLL void geode_color_color(const char* id, uint8_t* r, uint8_t* g, uint8_t* b, uint8_t* a);
    RUSTAPI_DLL void geode_color_color3b(const char* id, uint8_t* r, uint8_t* g, uint8_t* b);

    // cocos color utilities
    RUSTAPI_DLL void geode_cocos_invert3b(uint8_t r, uint8_t g, uint8_t b, uint8_t* or_, uint8_t* og, uint8_t* ob);
    RUSTAPI_DLL void geode_cocos_lighten3b(uint8_t r, uint8_t g, uint8_t b, int amount, uint8_t* or_, uint8_t* og, uint8_t* ob);
    RUSTAPI_DLL void geode_cocos_darken3b(uint8_t r, uint8_t g, uint8_t b, int amount, uint8_t* or_, uint8_t* og, uint8_t* ob);
    RUSTAPI_DLL bool geode_cocos_cc3b_from_hex(const char* hex, uint8_t* r, uint8_t* g, uint8_t* b);
    RUSTAPI_DLL bool geode_cocos_cc4b_from_hex(const char* hex, uint8_t* r, uint8_t* g, uint8_t* b, uint8_t* a);
    RUSTAPI_DLL void geode_cocos_invert4b(uint8_t r, uint8_t g, uint8_t b, uint8_t a, uint8_t* or_, uint8_t* og, uint8_t* ob, uint8_t* oa);
    RUSTAPI_DLL void geode_cocos_to3b(uint8_t r, uint8_t g, uint8_t b, uint8_t a, uint8_t* or_, uint8_t* og, uint8_t* ob);
    RUSTAPI_DLL void geode_cocos_to4b(uint8_t r, uint8_t g, uint8_t b, uint8_t alpha, uint8_t* or_, uint8_t* og, uint8_t* ob, uint8_t* oa);
    RUSTAPI_DLL void geode_cocos_to4f(uint8_t r, uint8_t g, uint8_t b, uint8_t a, float* or_, float* og, float* ob, float* oa);
    RUSTAPI_DLL char* geode_cocos_cc3b_to_hex(uint8_t r, uint8_t g, uint8_t b);
    RUSTAPI_DLL char* geode_cocos_cc4b_to_hex(uint8_t r, uint8_t g, uint8_t b, uint8_t a);


    // keybind
    RUSTAPI_DLL char* geode_keybind_from_string(const char* str);
    RUSTAPI_DLL char* geode_keybind_to_string(int key, int modifiers);

    // imgui
    RUSTAPI_DLL void imgui_init_host();
    RUSTAPI_DLL void imgui_shutdown_host();
    RUSTAPI_DLL void imgui_set_visible(bool visible);
    RUSTAPI_DLL void imgui_toggle();
    RUSTAPI_DLL bool imgui_is_visible();
    
    typedef void (*RustImGuiDrawCallback)();
    RUSTAPI_DLL void imgui_register_draw_callback(RustImGuiDrawCallback cb);

    RUSTAPI_DLL bool imgui_begin_window(const char* name);
    RUSTAPI_DLL void imgui_end_window();
    RUSTAPI_DLL bool imgui_begin_child(const char* str_id);
    RUSTAPI_DLL void imgui_end_child();

    RUSTAPI_DLL void imgui_text(const char* text);
    RUSTAPI_DLL void imgui_text_wrapped(const char* text);
    RUSTAPI_DLL void imgui_bullet_text(const char* text);
    RUSTAPI_DLL bool imgui_button(const char* label);
    RUSTAPI_DLL bool imgui_checkbox(const char* label, bool* v);
    RUSTAPI_DLL bool imgui_radio_button(const char* label, bool active);
    RUSTAPI_DLL bool imgui_slider_float(const char* label, float* v, float v_min, float v_max);
    RUSTAPI_DLL bool imgui_slider_int(const char* label, int* v, int v_min, int v_max);
    RUSTAPI_DLL bool imgui_drag_float(const char* label, float* v, float v_speed, float v_min, float v_max);
    RUSTAPI_DLL bool imgui_drag_int(const char* label, int* v, float v_speed, int v_min, int v_max);

    RUSTAPI_DLL void imgui_same_line();
    RUSTAPI_DLL void imgui_separator();
    RUSTAPI_DLL void imgui_separator_text(const char* text);
    RUSTAPI_DLL void imgui_spacing();
    RUSTAPI_DLL void imgui_new_line();
    RUSTAPI_DLL void imgui_indent(float w);
    RUSTAPI_DLL void imgui_unindent(float w);
    RUSTAPI_DLL void imgui_dummy(float x, float y);
    RUSTAPI_DLL void imgui_columns(int count, const char* id, bool border);
    RUSTAPI_DLL void imgui_next_column();
    RUSTAPI_DLL bool imgui_collapsing_header(const char* label);
    RUSTAPI_DLL bool imgui_tree_node(const char* label);
    RUSTAPI_DLL void imgui_tree_pop();
    RUSTAPI_DLL void imgui_begin_group();
    RUSTAPI_DLL void imgui_end_group();

    RUSTAPI_DLL void imgui_push_id_str(const char* id);
    RUSTAPI_DLL void imgui_push_id_int(int id);
    RUSTAPI_DLL void imgui_pop_id();

    RUSTAPI_DLL bool imgui_begin_table(const char* id, int columns);
    RUSTAPI_DLL void imgui_end_table();
    RUSTAPI_DLL void imgui_table_next_row();
    RUSTAPI_DLL bool imgui_table_next_column();
    RUSTAPI_DLL void imgui_table_setup_column(const char* label);
    RUSTAPI_DLL void imgui_table_headers_row();
    RUSTAPI_DLL bool imgui_table_set_column_index(int column);

    RUSTAPI_DLL void imgui_open_popup(const char* id);
    RUSTAPI_DLL bool imgui_begin_popup(const char* id);
    RUSTAPI_DLL void imgui_end_popup();
    RUSTAPI_DLL bool imgui_begin_popup_modal(const char* name);
    RUSTAPI_DLL void imgui_close_current_popup();
    RUSTAPI_DLL bool imgui_begin_tab_bar(const char* id);
    RUSTAPI_DLL void imgui_end_tab_bar();
    RUSTAPI_DLL bool imgui_begin_tab_item(const char* label);
    RUSTAPI_DLL void imgui_end_tab_item();
    RUSTAPI_DLL void imgui_set_tooltip(const char* text);

    RUSTAPI_DLL bool imgui_begin_menu_bar();
    RUSTAPI_DLL void imgui_end_menu_bar();
    RUSTAPI_DLL bool imgui_begin_menu(const char* label);
    RUSTAPI_DLL void imgui_end_menu();
    RUSTAPI_DLL bool imgui_menu_item(const char* label);

    RUSTAPI_DLL void imgui_style_colors_dark();
    RUSTAPI_DLL void imgui_style_colors_light();
    RUSTAPI_DLL void imgui_style_colors_classic();
    RUSTAPI_DLL void imgui_push_style_var_float(int idx, float val);
    RUSTAPI_DLL void imgui_push_style_var_vec2(int idx, float x, float y);
    RUSTAPI_DLL void imgui_pop_style_var(int count);
    RUSTAPI_DLL void imgui_push_style_color(int idx, float r, float g, float b, float a);
    RUSTAPI_DLL void imgui_pop_style_color(int count);

    // web 
    RUSTAPI_DLL void geode_web_open_link(const char* url);
    RUSTAPI_DLL void* geode_web_request_new();
    RUSTAPI_DLL void geode_web_request_free(void* handle);
    RUSTAPI_DLL void geode_web_request_header(void* handle, const char* name, const char* value);
    RUSTAPI_DLL void geode_web_request_param(void* handle, const char* name, const char* value);
    RUSTAPI_DLL void geode_web_request_method(void* handle, const char* method);
    RUSTAPI_DLL void geode_web_request_timeout(void* handle, int seconds);
    RUSTAPI_DLL void geode_web_request_body_string(void* handle, const char* data);
    RUSTAPI_DLL void geode_web_request_body_json(void* handle, const char* json);
    
    typedef void (*RustWebRequestCallback)(int status, const char* headers_json, const char* body, const char* error, void* user_data);
    RUSTAPI_DLL void geode_web_request_send(void* handle, const char* method, const char* url, RustWebRequestCallback cb, void* user_data);

    // cocos nodes
    RUSTAPI_DLL void cocos_node_set_pos(void* node, float x, float y);
    RUSTAPI_DLL void cocos_node_get_pos(void* node, float* x, float* y);
    RUSTAPI_DLL void cocos_node_set_scale(void* node, float s);
    RUSTAPI_DLL float cocos_node_get_scale(void* node);
    RUSTAPI_DLL void cocos_node_set_rot(void* node, float r);
    RUSTAPI_DLL float cocos_node_get_rot(void* node);
    RUSTAPI_DLL void cocos_node_set_visible(void* node, bool v);
    RUSTAPI_DLL bool cocos_node_is_visible(void* node);
    RUSTAPI_DLL void cocos_node_set_opacity(void* node, uint8_t o);
    RUSTAPI_DLL uint8_t cocos_node_get_opacity(void* node);
    RUSTAPI_DLL void cocos_node_set_color(void* node, uint8_t r, uint8_t g, uint8_t b);
    RUSTAPI_DLL void cocos_node_get_color(void* node, uint8_t* r, uint8_t* g, uint8_t* b);
    RUSTAPI_DLL void geode_node_add_child(void* node, void* child);
    RUSTAPI_DLL void geode_node_remove_from_parent(void* node);

    RUSTAPI_DLL void cocos_node_set_id(void* node, const char* id);
    RUSTAPI_DLL char* cocos_node_get_id(void* node);
    RUSTAPI_DLL int cocos_node_get_z_order(void* node);
    RUSTAPI_DLL void cocos_node_set_z_order(void* node, int z);
    RUSTAPI_DLL int cocos_node_get_child_count(void* node);
    RUSTAPI_DLL void cocos_node_set_anchor(void* node, float x, float y);
    RUSTAPI_DLL void cocos_node_get_anchor(void* node, float* x, float* y);
    RUSTAPI_DLL void cocos_node_set_content_size(void* node, float w, float h);
    RUSTAPI_DLL void cocos_node_get_content_size(void* node, float* w, float* h);
    RUSTAPI_DLL void cocos_get_win_size(float* w, float* h);
    RUSTAPI_DLL void geode_cocos_calculate_child_coverage(void* node, float* x, float* y, float* w, float* h);
    RUSTAPI_DLL void geode_cocos_calculate_node_coverage(void** nodes, int count, float* x, float* y, float* w, float* h);
    RUSTAPI_DLL void* geode_cocos_switch_to_scene(void* layer);
    RUSTAPI_DLL bool geode_cocos_node_is_visible(void* node);
    RUSTAPI_DLL void* geode_cocos_get_child_by_tag_recursive(void* node, int tag);
    RUSTAPI_DLL bool geode_cocos_is_sprite_frame_name(void* node, const char* name);
    RUSTAPI_DLL void* geode_cocos_get_child_by_sprite_frame_name(void* node, const char* name);
    RUSTAPI_DLL bool geode_cocos_is_sprite_name(void* node, const char* name);
    RUSTAPI_DLL void* geode_cocos_get_child_by_sprite_name(void* node, const char* name);
    RUSTAPI_DLL void geode_cocos_get_mouse_pos(float* x, float* y);
    RUSTAPI_DLL void geode_cocos_get_label_size(const char* text, const char* font, float kerning, float* w, float* h);
    RUSTAPI_DLL bool geode_cocos_file_exists_in_search_paths(const char* filename);
    RUSTAPI_DLL void geode_cocos_limit_node_size(void* node, float w, float h, float def, float min);
    RUSTAPI_DLL void geode_cocos_limit_node_width(void* node, float width, float def, float min);
    RUSTAPI_DLL void geode_cocos_limit_node_height(void* node, float height, float def, float min);


    // time and tasks or tasks and tiem idk sayhow u want
    RUSTAPI_DLL uint64_t geode_task_delay(double seconds, void (*cb)(uint64_t id, void* data), void* data);
    RUSTAPI_DLL uint64_t geode_task_every(double seconds, void (*cb)(uint64_t id, void* data), void* data);
    RUSTAPI_DLL void geode_task_cancel(uint64_t id);
    RUSTAPI_DLL void geode_task_run_main(void (*cb)(void*), void* data);
    RUSTAPI_DLL double geode_time_now();
    RUSTAPI_DLL double geode_time_unix();

    // platform
    RUSTAPI_DLL bool geode_platform_is_windows();
    RUSTAPI_DLL bool geode_platform_is_macos();
    RUSTAPI_DLL bool geode_platform_is_ios();
    RUSTAPI_DLL bool geode_platform_is_android();

    // string free
    RUSTAPI_DLL void geode_free_string(char* s);
}

