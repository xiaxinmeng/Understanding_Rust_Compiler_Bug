
error[E0433]: failed to resolve. Could not find `_core` in `$crate`
   --> src/librustc_llvm/ffi.rs:455:5
    |
455 | /     bitflags! {
456 | |         #[repr(C)]
457 | |         #[derive(Default)]
458 | |         pub struct DIFlags: ::libc::uint32_t {
...   |
477 | |         }
478 | |     }
    | |_____^ Could not find `_core` in `$crate`
    |
    = note: this error originates in a macro outside of the current crate

error[E0433]: failed to resolve. Could not find `_core` in `$crate`
   --> src/librustc_llvm/ffi.rs:455:5
    |
455 | /     bitflags! {
456 | |         #[repr(C)]
457 | |         #[derive(Default)]
458 | |         pub struct DIFlags: ::libc::uint32_t {
...   |
477 | |         }
478 | |     }
    | |_____^ Could not find `_core` in `$crate`
    |
    = note: this error originates in a macro outside of the current crate

error[E0433]: failed to resolve. Could not find `_core` in `$crate`
