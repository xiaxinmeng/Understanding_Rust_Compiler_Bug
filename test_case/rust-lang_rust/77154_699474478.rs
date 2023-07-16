rust
#![cfg_attr(target_os = "emscripten", feature(link_args))]
#[cfg(target_os = "emscripten")]
#[allow(unused_attributes)]
#[link_args = "-s EXIT_RUNTIME=1"]
extern "C" {}
