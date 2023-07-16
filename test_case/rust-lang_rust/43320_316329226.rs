rust
// src/librustc_llvm/build.rs
    let llvm_config = env::var_os("LLVM_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            if let Some(dir) = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from) {
                let to_test = dir.parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(&target)
                    .join("llvm/bin/llvm-config");
                if Command::new(&to_test).output().is_ok() {
                    return to_test;
                }
            }
            PathBuf::from("llvm-config")
        });
...
    let mut version_cmd = Command::new(&llvm_config);
    version_cmd.arg("--version");
