#include "helpers.hpp"
#include <imgui.h>
#include <imgui-cocos.hpp>

extern "C" {
    // core
    RUSTAPI_DLL void imgui_init_host() {
        ImGuiCocos::get().setup([] {}).draw([] {
            // we will call rust draw callbacks here if we had any
        });
    }

    RUSTAPI_DLL void imgui_shutdown_host() {
        if (ImGuiCocos::get().isInitialized()) {
            ImGuiCocos::get().destroy();
        }
    }

    RUSTAPI_DLL void imgui_set_visible(bool visible) {
        ImGuiCocos::get().setVisible(visible);
    }

    RUSTAPI_DLL void imgui_toggle() {
        ImGuiCocos::get().toggle();
    }

    RUSTAPI_DLL bool imgui_is_visible() {
        return ImGuiCocos::get().isVisible();
    }

    // a simple callback for the draw loop
    typedef void (*RustImGuiDrawCallback)();
    static RustImGuiDrawCallback g_drawCb = nullptr;

    RUSTAPI_DLL void imgui_register_draw_callback(RustImGuiDrawCallback cb) {
        g_drawCb = cb;
        ImGuiCocos::get().setup([] {}).draw([] {
            if (g_drawCb) {
                g_drawCb();
            }
        });
    }

    // windows
    RUSTAPI_DLL bool imgui_begin_window(const char* name) { return ImGui::Begin(name); }
    RUSTAPI_DLL void imgui_end_window() { ImGui::End(); }
    RUSTAPI_DLL bool imgui_begin_child(const char* str_id) { return ImGui::BeginChild(str_id); }
    RUSTAPI_DLL void imgui_end_child() { ImGui::EndChild(); }

    // wdigets
    RUSTAPI_DLL void imgui_text(const char* text) { ImGui::TextUnformatted(text); }
    RUSTAPI_DLL void imgui_text_wrapped(const char* text) { ImGui::TextWrapped("%s", text); }
    RUSTAPI_DLL void imgui_bullet_text(const char* text) { ImGui::BulletText("%s", text); }
    RUSTAPI_DLL bool imgui_button(const char* label) { return ImGui::Button(label); }
    RUSTAPI_DLL bool imgui_checkbox(const char* label, bool* v) { return ImGui::Checkbox(label, v); }
    RUSTAPI_DLL bool imgui_radio_button(const char* label, bool active) { return ImGui::RadioButton(label, active); }
    
    RUSTAPI_DLL bool imgui_slider_float(const char* label, float* v, float v_min, float v_max) { 
        return ImGui::SliderFloat(label, v, v_min, v_max); 
    }
    RUSTAPI_DLL bool imgui_slider_int(const char* label, int* v, int v_min, int v_max) { 
        return ImGui::SliderInt(label, v, v_min, v_max); 
    }
    RUSTAPI_DLL bool imgui_drag_float(const char* label, float* v, float v_speed, float v_min, float v_max) {
        return ImGui::DragFloat(label, v, v_speed, v_min, v_max);
    }
    RUSTAPI_DLL bool imgui_drag_int(const char* label, int* v, float v_speed, int v_min, int v_max) {
        return ImGui::DragInt(label, v, v_speed, v_min, v_max);
    }

    // layout
    RUSTAPI_DLL void imgui_same_line() { ImGui::SameLine(); }
    RUSTAPI_DLL void imgui_separator() { ImGui::Separator(); }
    RUSTAPI_DLL void imgui_separator_text(const char* text) { ImGui::SeparatorText(text); }
    RUSTAPI_DLL void imgui_spacing() { ImGui::Spacing(); }
    RUSTAPI_DLL void imgui_new_line() { ImGui::NewLine(); }
    RUSTAPI_DLL void imgui_indent(float w) { ImGui::Indent(w); }
    RUSTAPI_DLL void imgui_unindent(float w) { ImGui::Unindent(w); }
    RUSTAPI_DLL void imgui_dummy(float x, float y) { ImGui::Dummy({x, y}); }
    
    RUSTAPI_DLL void imgui_columns(int count, const char* id, bool border) { ImGui::Columns(count, id, border); }
    RUSTAPI_DLL void imgui_next_column() { ImGui::NextColumn(); }

    RUSTAPI_DLL bool imgui_collapsing_header(const char* label) { return ImGui::CollapsingHeader(label); }
    RUSTAPI_DLL bool imgui_tree_node(const char* label) { return ImGui::TreeNode(label); }
    RUSTAPI_DLL void imgui_tree_pop() { ImGui::TreePop(); }
    RUSTAPI_DLL void imgui_begin_group() { ImGui::BeginGroup(); }
    RUSTAPI_DLL void imgui_end_group() { ImGui::EndGroup(); }

    // ids
    RUSTAPI_DLL void imgui_push_id_str(const char* id) { ImGui::PushID(id); }
    RUSTAPI_DLL void imgui_push_id_int(int id) { ImGui::PushID(id); }
    RUSTAPI_DLL void imgui_pop_id() { ImGui::PopID(); }

    // tables
    RUSTAPI_DLL bool imgui_begin_table(const char* id, int columns) { return ImGui::BeginTable(id, columns); }
    RUSTAPI_DLL void imgui_end_table() { ImGui::EndTable(); }
    RUSTAPI_DLL void imgui_table_next_row() { ImGui::TableNextRow(); }
    RUSTAPI_DLL bool imgui_table_next_column() { return ImGui::TableNextColumn(); }
    RUSTAPI_DLL void imgui_table_setup_column(const char* label) { ImGui::TableSetupColumn(label); }
    RUSTAPI_DLL void imgui_table_headers_row() { ImGui::TableHeadersRow(); }
    RUSTAPI_DLL bool imgui_table_set_column_index(int column) { return ImGui::TableSetColumnIndex(column); }

    // popups
    RUSTAPI_DLL void imgui_open_popup(const char* id) { ImGui::OpenPopup(id); }
    RUSTAPI_DLL bool imgui_begin_popup(const char* id) { return ImGui::BeginPopup(id); }
    RUSTAPI_DLL void imgui_end_popup() { ImGui::EndPopup(); }
    RUSTAPI_DLL bool imgui_begin_popup_modal(const char* name) { return ImGui::BeginPopupModal(name); }
    RUSTAPI_DLL void imgui_close_current_popup() { ImGui::CloseCurrentPopup(); }
    RUSTAPI_DLL bool imgui_begin_tab_bar(const char* id) { return ImGui::BeginTabBar(id); }
    RUSTAPI_DLL void imgui_end_tab_bar() { ImGui::EndTabBar(); }
    RUSTAPI_DLL bool imgui_begin_tab_item(const char* label) { return ImGui::BeginTabItem(label); }
    RUSTAPI_DLL void imgui_end_tab_item() { ImGui::EndTabItem(); }
    RUSTAPI_DLL void imgui_set_tooltip(const char* text) { ImGui::SetTooltip("%s", text); }

    // menus
    RUSTAPI_DLL bool imgui_begin_menu_bar() { return ImGui::BeginMenuBar(); }
    RUSTAPI_DLL void imgui_end_menu_bar() { ImGui::EndMenuBar(); }
    RUSTAPI_DLL bool imgui_begin_menu(const char* label) { return ImGui::BeginMenu(label); }
    RUSTAPI_DLL void imgui_end_menu() { ImGui::EndMenu(); }
    RUSTAPI_DLL bool imgui_menu_item(const char* label) { return ImGui::MenuItem(label); }

    // style and theme
    RUSTAPI_DLL void imgui_style_colors_dark() { ImGui::StyleColorsDark(); }
    RUSTAPI_DLL void imgui_style_colors_light() { ImGui::StyleColorsLight(); }
    RUSTAPI_DLL void imgui_style_colors_classic() { ImGui::StyleColorsClassic(); }
    RUSTAPI_DLL void imgui_push_style_var_float(int idx, float val) { ImGui::PushStyleVar(idx, val); }
    RUSTAPI_DLL void imgui_push_style_var_vec2(int idx, float x, float y) { ImGui::PushStyleVar(idx, {x, y}); }
    RUSTAPI_DLL void imgui_pop_style_var(int count) { ImGui::PopStyleVar(count); }
    RUSTAPI_DLL void imgui_push_style_color(int idx, float r, float g, float b, float a) { ImGui::PushStyleColor(idx, {r, g, b, a}); }
    RUSTAPI_DLL void imgui_pop_style_color(int count) { ImGui::PopStyleColor(count); }
}
