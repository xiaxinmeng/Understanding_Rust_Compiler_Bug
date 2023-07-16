Rust
mod hawktracer {
  cfg_if::cfg_if! {
    // Do not mix tracing and tests
    if #[cfg(all(feature="tracing", not(test), not(doctest)))] {
      pub use rust_hawktracer::*;
    } else {
      pub use noop_proc_macro::hawktracer;
    }
  }
}
