rust
#[cfg(all(crate_type = "cdylib", any(crate_type = "staticlib", ...)))]
compile_fail!(...);
