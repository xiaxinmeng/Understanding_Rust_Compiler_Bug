 rust
#[cfg(debug_assertions)]
extern crate oom_report; // for development, verbose

#[cfg(debug_assertions)]
extern crate oom_panic; // for release, minimal size
