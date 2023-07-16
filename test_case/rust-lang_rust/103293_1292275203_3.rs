Rust
        let symlink_target = PathBuf::from(&out_dir).join("libbinaryninjacore.so");
        if !link_path.join("libbinaryninjacore.so").exists() && !symlink_target.exists() {
            use std::os::unix::fs;
            fs::symlink(
                link_path.join("libbinaryninjacore.so.1"),
                PathBuf::from(&out_dir).join("libbinaryninjacore.so"),
            )
            .expect("failed to create required symlink");
        }
