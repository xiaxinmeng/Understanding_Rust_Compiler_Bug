plain
[RUSTC-TIMING] gimli test:false 5.575
[RUSTC-TIMING] object test:false 5.737
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error[E0432]: unresolved import `mutex::ReentrantMutex`
 --> library/std/src/sys/wasm/../unsupported/locks/mod.rs:5:38
  |
5 | pub use mutex::{MovableMutex, Mutex, ReentrantMutex};
  |                                      ^^^^^^^^^^^^^^ no `ReentrantMutex` in `sys::wasm::locks::mutex`
For more information about this error, try `rustc --explain E0432`.
[RUSTC-TIMING] std test:false 2.001
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
