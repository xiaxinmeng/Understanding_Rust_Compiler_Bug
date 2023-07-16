 rust
// This module hides some optional stuff
#[cfg(foo_bar)]
#[path = ""]
mod thread {
    #[path = "src/tls.rs"]
    mod local_data;
}
