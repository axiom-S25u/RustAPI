use std::ffi::CString;
use rustapi::bindings::geode::{log_info, log_warn, log_error, thread, platform, random, ui, game};
use rustapi::bindings::imgui;

extern "C" {
    fn geode_create_alert_popup(
        title: *const std::os::raw::c_char,
        content: *const std::os::raw::c_char,
        btn1: *const std::os::raw::c_char,
        btn2: *const std::os::raw::c_char,
        callback: extern "C" fn(bool)
    );
    fn geode_create_quick_popup(
        title: *const std::os::raw::c_char,
        content: *const std::os::raw::c_char,
        btn1: *const std::os::raw::c_char,
        btn2: *const std::os::raw::c_char,
        callback: extern "C" fn(bool)
    );
}

static mut POPUP_CALLBACK: Option<Box<dyn Fn(bool) + Send>> = None;

extern "C" fn popup_callback(clicked_btn2: bool) {
    unsafe {
        if let Some(ref cb) = POPUP_CALLBACK {
            cb(clicked_btn2);
        }
    }
}

fn create_alert_popup(title: &str, content: &str, btn1: &str, btn2: &str, callback: impl Fn(bool) + Send + 'static) {
    let c_title = CString::new(title).unwrap();
    let c_content = CString::new(content).unwrap();
    let c_btn1 = CString::new(btn1).unwrap();
    let c_btn2 = CString::new(btn2).unwrap();

    unsafe {
        POPUP_CALLBACK = Some(Box::new(callback));
        geode_create_alert_popup(
            c_title.as_ptr(),
            c_content.as_ptr(),
            c_btn1.as_ptr(),
            c_btn2.as_ptr(),
            popup_callback
        );
    }
}

fn create_quick_popup(title: &str, content: &str, btn1: &str, btn2: &str, callback: impl Fn(bool) + Send + 'static) {
    let c_title = CString::new(title).unwrap();
    let c_content = CString::new(content).unwrap();
    let c_btn1 = CString::new(btn1).unwrap();
    let c_btn2 = CString::new(btn2).unwrap();

    unsafe {
        POPUP_CALLBACK = Some(Box::new(callback));
        geode_create_quick_popup(
            c_title.as_ptr(),
            c_content.as_ptr(),
            c_btn1.as_ptr(),
            c_btn2.as_ptr(),
            popup_callback
        );
    }
}

#[no_mangle]
pub extern "C" fn testmod_init() {
    log_info("rustapi testmod loaded from rust");
    log_warn("this is a warning from testmod");
    log_error("this is an error from testmod");

    // test the new bindings
    log_info(&format!("thread name: {}", thread::get_name()));
    log_info(&format!("is wine: {}", platform::is_wine()));
    log_info(&format!("random uuid: {}", random::generate_uuid()));


    rustapi::bindings::imgui::ImGuiContext::register_draw_callback(my_draw_cb);
}

extern "C" fn my_draw_cb() {
    let imgui = rustapi::bindings::imgui::ImGuiContext::new();
    if imgui.begin_window("Testmod ImGui") {
        imgui.text("Hello from testmod!");
        if imgui.button("Print thread name") {
            log_info(&format!("Thread from ImGui: {}", thread::get_name()));
        }
        if imgui.button("Test Alert Popup (FLAlertLayer)") {
            create_alert_popup(
                "ImGui Alert",
                "This popup was opened from ImGui using Rust!",
                "Cool",
                "Whatever",
                |btn2| {
                    log_info(&format!("Alert closed. Clicked btn2: {}", btn2));
                }
            );
        }
        if imgui.button("Test Quick Popup") {
            create_quick_popup(
                "ImGui Quick",
                "This quick popup was triggered by ImGui button click",
                "Nice",
                "Cancel",
                |btn2| {
                    log_info(&format!("Quick Popup closed. Clicked btn2: {}", btn2));
                }
            );
        }
        if imgui.button("Test Random UUID") {
            log_info(&format!("New UUID: {}", random::generate_uuid()));
        }
        if imgui.button("Test Random String (Length 10)") {
            log_info(&format!("Random string: {}", random::generate_alphanumeric_string(10)));
        }
        if imgui.button("Open Mods List") {
            ui::open_mods_list();
            log_info("Opened mods list");
        }
        if imgui.button("Exit Game") {
            game::exit();
        }
        imgui.end_window();
    }
}

#[no_mangle]
pub extern "C" fn testmod_shutdown() {
    log_info("rustapi testmod is shutting down");
}
