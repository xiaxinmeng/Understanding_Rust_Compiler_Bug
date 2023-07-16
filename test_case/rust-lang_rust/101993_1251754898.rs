rust
            prepare("clippy");
            for tool in &["rust-demangler", "rust-analyzer", "miri"] {
                if built_tools.contains(tool) {
                    prepare(tool);
                }
            }
            if target.ends_with("windows-gnu") {
                prepare("rust-mingw");
            }
