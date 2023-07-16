 rust
#![crate_type="dylib"]
// Any large (>8MB AFAICT) file works, librustc_llvm happens to take up 44MB.
pub const BLOB: &'static [u8] =
include_bytes!("build/x86_64-unknown-linux-gnu/stage0/lib/librustc_llvm-a4922355.so");
