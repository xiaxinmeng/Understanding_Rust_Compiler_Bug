Rust
#[cfg_attr(not(windows), link(name = "dylib"))]
#[cfg_attr(windows, link(name = "dylib.dll"))]
