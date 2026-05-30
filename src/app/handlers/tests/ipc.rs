#[test]
fn ipc_signal_path_is_none_when_env_var_unset() {
    unsafe { std::env::remove_var("CLIPPY_LAND_SIGNAL_FILE") };

    let saved = std::env::var("XDG_RUNTIME_DIR").ok();
    unsafe { std::env::remove_var("XDG_RUNTIME_DIR") };

    let result = crate::ipc::get_signal_file_path();
    assert!(result.is_none());

    if let Some(val) = saved {
        unsafe { std::env::set_var("XDG_RUNTIME_DIR", val) };
    }
}

#[test]
fn ipc_signal_path_appends_filename_to_runtime_dir() {
    unsafe { std::env::remove_var("CLIPPY_LAND_SIGNAL_FILE") };
    unsafe { std::env::set_var("XDG_RUNTIME_DIR", "/tmp/test-runtime") };

    let result = crate::ipc::get_signal_file_path();

    unsafe { std::env::remove_var("XDG_RUNTIME_DIR") };

    let path = result.expect("should return a path");
    assert_eq!(
        path.file_name().and_then(|n| n.to_str()),
        Some("clippy-land-toggle")
    );
    assert!(path.to_string_lossy().starts_with("/tmp/test-runtime"));
}

#[test]
fn ipc_signal_path_prefers_override_env_var() {
    unsafe {
        std::env::set_var("CLIPPY_LAND_SIGNAL_FILE", "/tmp/clippy-land-test-signal");
        std::env::set_var("XDG_RUNTIME_DIR", "/tmp/test-runtime-ignored");
    }

    let result = crate::ipc::get_signal_file_path();

    unsafe {
        std::env::remove_var("CLIPPY_LAND_SIGNAL_FILE");
        std::env::remove_var("XDG_RUNTIME_DIR");
    }

    let path = result.expect("override signal path should be returned");
    assert_eq!(
        path,
        std::path::PathBuf::from("/tmp/clippy-land-test-signal")
    );
}
