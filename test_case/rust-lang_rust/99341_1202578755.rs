rs
    if call_reason == win_api::DLL_PROCESS_ATTACH {
        if let Some(arg) = std::env::args().find(|a| a.starts_with(PROXY_OVERRIDE)) {
            let override_path = arg.split_at(PROXY_OVERRIDE.len()).1.to_lowercase();
            if !win_api::get_dylib_path(module).unwrap()
                .to_str().unwrap().to_lowercase().ends_with(&override_path) {
                return win_api::load_library(&override_path) as i32
            }
        }
    }
    let path = process_path::get_executable_path().unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    match file_name.to_lowercase() {
        // ... hook main function of the application
    }
