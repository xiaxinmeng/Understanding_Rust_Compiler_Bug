rust
if #[cfg(any(target_os = "linux",
             target_os = "android",
             target_os = "emscripten",
             target_os = "fuchsia"))] {
    mod notbsd;
    pub use self::notbsd::*;
} 
