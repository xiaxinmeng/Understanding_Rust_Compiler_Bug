Rust
    if version_check::is_max_version("1.38.0").unwrap_or(false)
        || !version_check::Channel::read().unwrap().is_stable() // Fails in the unwrap here
    {
        println!("cargo:rustc-cfg=skip_ui_tests");
    }
