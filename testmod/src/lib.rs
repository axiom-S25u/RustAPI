use std::ffi::CString;

extern "C" {
    fn geode_log_info(msg: *const std::os::raw::c_char);
    fn geode_log_warn(msg: *const std::os::raw::c_char);
    fn geode_log_error(msg: *const std::os::raw::c_char);
    fn geode_log_debug(msg: *const std::os::raw::c_char);

    fn geode_create_alert_popup(
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

fn log_info(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { geode_log_info(c_msg.as_ptr()) };
}

fn log_warn(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { geode_log_warn(c_msg.as_ptr()) };
}

fn log_error(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { geode_log_error(c_msg.as_ptr()) };
}

fn log_debug(msg: &str) {
    let c_msg = CString::new(msg).unwrap();
    unsafe { geode_log_debug(c_msg.as_ptr()) };
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

#[no_mangle]
pub extern "C" fn testmod_init() {
    log_info("rustapi testmod loaded from rust");
    log_warn("this is a warning from testmod");
    log_error("this is an error from testmod");
    log_debug("this is a debug message from testmod");

    create_alert_popup(
        "test alert",
        "hello from rustapi testmod",
        "ok",
        "cancel",
        |clicked_btn2| {
            if clicked_btn2 {
                log_info("user clicked cancel");
            } else {
                log_info("user clicked ok");
            }
        }
    );
}

#[no_mangle]
pub extern "C" fn testmod_shutdown() {
    log_info("rustapi testmod is shutting down");
}
