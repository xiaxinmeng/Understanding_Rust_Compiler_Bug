rust
#[cfg(target_os = "illumos")]
#[link(name = "gcc_s")]
#[link(name = "c")]
extern "C" {}
