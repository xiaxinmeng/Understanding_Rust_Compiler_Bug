 rust
// explain that this is what provides the gcc_personality function and libunwind interface in this comment.
#[cfg(target_os = "macos")]
#[link(name = "gcc_s")]
extern {}
#[cfg(not(target_os = "macos"))]
#[link(name = "gcc")]
extern {}
