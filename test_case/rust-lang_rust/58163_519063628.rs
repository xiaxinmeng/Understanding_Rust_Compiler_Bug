rust
#[link(name = "c", kind = "static", cfg(target_feature = "crt-static"))]
#[link(name = "c", cfg(not(target_feature = "crt-static")))]
extern {}
