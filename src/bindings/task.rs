use crate::ffi;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub type TaskCallback = Box<dyn FnOnce() + Send + 'static>;

pub fn run_on_main(callback: TaskCallback) {
    let cb_box = Box::new(callback);
    let ptr = Box::into_raw(cb_box);

    extern "C" fn tramp(data: *mut c_void) {
        unsafe {
            let cb_box = Box::from_raw(data as *mut TaskCallback);
            (*cb_box)();
        }
    }

    unsafe { ffi::geode_task_run_main(tramp, ptr as *mut c_void); }
}

pub fn spawn(callback: TaskCallback) {
    run_on_main(callback);
}

pub fn defer(callback: TaskCallback) {
    run_on_main(callback);
}

struct RegisteredTask {
    cb: Arc<Mutex<dyn FnMut(u64) + Send>>,
    is_repeating: bool,
}

lazy_static::lazy_static! {
    static ref ACTIVE_TASKS: Mutex<HashMap<u64, RegisteredTask>> = Mutex::new(HashMap::new());
}

extern "C" fn generic_task_cb(id: u64, _data: *mut c_void) {
    let task_opt = {
        if let Ok(mut map) = ACTIVE_TASKS.lock() {
            if let Some(task) = map.get(&id) {
                let arc = task.cb.clone();
                let is_repeating = task.is_repeating;
                if !is_repeating {
                    map.remove(&id);
                }
                Some((arc, is_repeating))
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some((cb_arc, _is_repeating)) = task_opt {
        if let Ok(mut cb) = cb_arc.lock() {
            cb(id);
        }
    }
}

pub fn delay<F>(seconds: f64, callback: F) -> u64
where
    F: FnOnce(u64) + Send + 'static,
{
    let mut cb_opt = Some(callback);
    let cb = Arc::new(Mutex::new(move |id| {
        if let Some(c) = cb_opt.take() {
            c(id);
        }
    }));
    
    unsafe {
        let id = ffi::geode_task_delay(seconds, generic_task_cb, std::ptr::null_mut());
        if let Ok(mut map) = ACTIVE_TASKS.lock() {
            map.insert(id, RegisteredTask { cb, is_repeating: false });
        }
        id
    }
}

pub fn every<F>(seconds: f64, callback: F) -> u64
where
    F: FnMut(u64) + Send + 'static,
{
    let cb = Arc::new(Mutex::new(callback));
    unsafe {
        let id = ffi::geode_task_every(seconds, generic_task_cb, std::ptr::null_mut());
        if let Ok(mut map) = ACTIVE_TASKS.lock() {
            map.insert(id, RegisteredTask { cb, is_repeating: true });
        }
        id
    }
}

pub fn cancel(id: u64) {
    unsafe {
        ffi::geode_task_cancel(id);
    }
    if let Ok(mut map) = ACTIVE_TASKS.lock() {
        map.remove(&id);
    }
}

pub fn time_now() -> f64 {
    unsafe { ffi::geode_time_now() }
}

pub fn time_unix() -> f64 {
    unsafe { ffi::geode_time_unix() }
}

pub mod time {
    use super::*;
    pub fn now() -> f64 {
        time_now()
    }
    pub fn unix() -> f64 {
        time_unix()
    }
}
