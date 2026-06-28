#[cfg(not(test))]
mod ffi;
mod runtime;
#[cfg(not(test))]
pub mod bindings;
#[cfg(test)]
mod tests;

#[cfg(not(test))]
use std::ffi::{CStr, CString};
#[cfg(not(test))]
use std::os::raw::c_char;
#[cfg(not(test))]
use std::sync::Mutex;

#[cfg(not(test))]
use runtime::RustRuntime;

#[cfg(not(test))]
lazy_static::lazy_static! {
    static ref RUNTIME: Mutex<RustRuntime> = Mutex::new(RustRuntime::new());
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rustapi_init() -> *mut c_char {
    let mut runtime = RUNTIME.lock().unwrap();
    match runtime.init() {
        Ok(_) => std::ptr::null_mut(),
        Err(e) => {
            let err = CString::new(e).unwrap();
            err.into_raw()
        }
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rustapi_shutdown() {
    let mut runtime = RUNTIME.lock().unwrap();
    runtime.shutdown();
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rustapi_run_script(
    source: *const c_char,
    chunk_name: *const c_char,
) -> *mut c_char {
    if source.is_null() || chunk_name.is_null() {
        let err = CString::new("null pointer").unwrap();
        return err.into_raw();
    }

    let source_str = unsafe { CStr::from_ptr(source) }.to_str().unwrap().to_string();
    let chunk_str = unsafe { CStr::from_ptr(chunk_name) }.to_str().unwrap().to_string();

    let mut runtime = RUNTIME.lock().unwrap();
    match runtime.run_script(source_str, chunk_str) {
        Ok(_) => std::ptr::null_mut(),
        Err(e) => {
            let err = CString::new(e).unwrap();
            err.into_raw()
        }
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rustapi_run_file(path: *const c_char) -> *mut c_char {
    if path.is_null() {
        let err = CString::new("null pointer").unwrap();
        return err.into_raw();
    }

    let path_str = unsafe { CStr::from_ptr(path) }.to_str().unwrap().to_string();

    let mut runtime = RUNTIME.lock().unwrap();
    match runtime.run_file(path_str) {
        Ok(_) => std::ptr::null_mut(),
        Err(e) => {
            let err = CString::new(e).unwrap();
            err.into_raw()
        }
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rustapi_is_ready() -> bool {
    let runtime = RUNTIME.lock().unwrap();
    runtime.is_initialized()
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rustapi_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
