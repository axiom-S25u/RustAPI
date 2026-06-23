// tests live here, no ffi, no game, no pain (well, less pain)

use crate::runtime::RustRuntime;

#[test]
fn runtime_starts_uninitialized() {
    let rt = RustRuntime::new();
    assert!(!rt.is_initialized());
}

#[test]
fn runtime_init_idempotent() {
    let mut rt = RustRuntime::new_headless();
    rt.init_headless();
    assert!(rt.is_initialized());
    rt.init_headless(); // second call shouldnt blow up or change state
    assert!(rt.is_initialized());
}

#[test]
fn runtime_shutdown_resets_state() {
    let mut rt = RustRuntime::new_headless();
    rt.init_headless();
    assert!(rt.is_initialized());
    rt.shutdown_headless();
    assert!(!rt.is_initialized());
}

#[test]
fn run_script_fails_before_init() {
    let mut rt = RustRuntime::new_headless();
    let result = rt.run_script("fn main() {}".to_string(), "test".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "runtime not initialized");
}

#[test]
fn run_file_fails_before_init() {
    let mut rt = RustRuntime::new_headless();
    let result = rt.run_file("/some/path.rs".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "runtime not initialized");
}

#[test]
fn run_script_succeeds_after_init() {
    let mut rt = RustRuntime::new_headless();
    rt.init_headless();
    let result = rt.run_script("let x = 1;".to_string(), "myscript".to_string());
    assert!(result.is_ok());
}

#[test]
fn run_file_succeeds_after_init() {
    let mut rt = RustRuntime::new_headless();
    rt.init_headless();
    let result = rt.run_file("/fake/path.rs".to_string());
    assert!(result.is_ok());
}

#[test]
fn shutdown_then_run_fails() {
    let mut rt = RustRuntime::new_headless();
    rt.init_headless();
    rt.shutdown_headless();
    let result = rt.run_script("let x = 1;".to_string(), "dead".to_string());
    assert!(result.is_err());
}

#[test]
fn script_registry_tracks_scripts() {
    let mut rt = RustRuntime::new_headless();
    rt.init_headless();
    rt.run_script("let a = 1;".to_string(), "script_a".to_string()).unwrap();
    rt.run_script("let b = 2;".to_string(), "script_b".to_string()).unwrap();
    assert_eq!(rt.script_count(), 2);
}

#[test]
fn shutdown_clears_scripts() {
    let mut rt = RustRuntime::new_headless();
    rt.init_headless();
    rt.run_script("let a = 1;".to_string(), "script_a".to_string()).unwrap();
    assert_eq!(rt.script_count(), 1);
    rt.shutdown_headless();
    assert_eq!(rt.script_count(), 0);
}
