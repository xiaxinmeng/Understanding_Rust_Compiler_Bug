plain
[01:28:59]    |
[01:28:59] LL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning
[01:28:59]    |                                           ^^^^^^ help: a function with a similar name exists: `init`
[01:28:59] 
[01:28:59] error: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[01:28:59]    |
[01:28:59] LL | use std::intrinsics::{init, uninit};
[01:28:59]    |                       ^^^^
[01:28:59]    |
[01:28:59]    |
[01:28:59]    = note: `-D deprecated` implied by `-D warnings`
[01:28:59] 
[01:28:59] error: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[01:28:59]    |
[01:28:59]    |
[01:28:59] LL |     let ref_zero: &T = std::intrinsics::init(); // warning
[01:28:59] 
[01:28:59] 
[01:28:59] error: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[01:28:59]    |
[01:28:59]    |
[01:28:59] LL |     let mem_zero: usize = std::intrinsics::init(); // no warning
[01:28:59] 
[01:28:59] error: aborting due to 5 previous errors
[01:28:59] 
[01:28:59] Some errors have detailed explanations: E0425, E0432.
[01:28:59] Some errors have detailed explanations: E0425, E0432.
[01:28:59] For more information about an error, try `rustc --explain E0425`.
[01:28:59] 
[01:28:59] 
[01:28:59] expected stderr:
[01:28:59] error: reference to zeroed memory
[01:28:59]   --> $DIR/invalid_ref.rs:24:24
[01:28:59]    |
[01:28:59] LL |     let ref_zero: &T = std::mem::zeroed(); // warning
[01:28:59]    |
[01:28:59]    = note: #[deny(clippy::invalid_ref)] on by default
[01:28:59]    = note: #[deny(clippy::invalid_ref)] on by default
[01:28:59]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59] error: reference to zeroed memory
[01:28:59]   --> $DIR/invalid_ref.rs:28:24
[01:28:59]    |
[01:28:59]    |
[01:28:59] LL |     let ref_zero: &T = core::mem::zeroed(); // warning
[01:28:59]    |
[01:28:59]    |
[01:28:59]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59] error: reference to zeroed memory
[01:28:59]   --> $DIR/invalid_ref.rs:32:24
[01:28:59]    |
[01:28:59]    |
[01:28:59] LL |     let ref_zero: &T = std::intrinsics::init(); // warning
[01:28:59]    |
[01:28:59]    |
[01:28:59]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59] error: reference to uninitialized memory
[01:28:59]   --> $DIR/invalid_ref.rs:36:26
[01:28:59]    |
[01:28:59] LL |     let ref_uninit: &T = std::mem::uninitialized(); // warning
[01:28:59] LL |     let ref_uninit: &T = std::mem::uninitialized(); // warning
[01:28:59]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:28:59]    |
[01:28:59]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59] error: reference to uninitialized memory
[01:28:59]   --> $DIR/invalid_ref.rs:40:26
[01:28:59]    |
[01:28:59] LL |     let ref_uninit: &T = core::mem::uninitialized(); // warning
[01:28:59] LL |     let ref_uninit: &T = core::mem::uninitialized(); // warning
[01:28:59]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:28:59]    |
[01:28:59]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59] error: reference to uninitialized memory
[01:28:59]   --> $DIR/invalid_ref.rs:44:26
[01:28:59]    |
[01:28:59] LL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning
[01:28:59] LL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning
[01:28:59]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:28:59]    |
[01:28:59]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59] error: aborting due to 6 previous errors
[01:28:59] 
[01:28:59] 
[01:28:59] 
[01:28:59] 
[01:28:59] diff of stderr:
[01:28:59] 
[01:28:59] -error: reference to zeroed memory
[01:28:59] -  --> $DIR/invalid_ref.rs:24:24
[01:28:59] +error[E0432]: unresolved import `std::intrinsics::uninit`
[01:28:59] +  --> $DIR/invalid_ref.rs:5:29
[01:28:59]     |
[01:28:59] -LL |     let ref_zero: &T = std::mem::zeroed(); // warning
[01:28:59] +LL | use std::intrinsics::{init, uninit};
[01:28:59] +   |                             ^^^^^^
[01:28:59] +   |                             |
[01:28:59] +   |                             no `uninit` in `intrinsics`
[01:28:59] +   |                             no `uninit` in `intrinsics`
[01:28:59] +   |                             help: a similar name exists in the module: `init`
[01:28:59] +
[01:28:59] +error[E0425]: cannot find function `uninit` in module `std::intrinsics`
[01:28:59] +  --> $DIR/invalid_ref.rs:44:43
[01:28:59]     |
[01:28:59] -   = note: #[deny(clippy::invalid_ref)] on by default
[01:28:59] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59] +LL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning
[01:28:59]  
[01:28:59] -error: reference to zeroed memory
[01:28:59] -  --> $DIR/invalid_ref.rs:28:24
[01:28:59] -  --> $DIR/invalid_ref.rs:28:24
[01:28:59] +error: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[01:28:59]     |
[01:28:59]     |
[01:28:59] -LL |     let ref_zero: &T = core::mem::zeroed(); // warning
[01:28:59] +LL | use std::intrinsics::{init, uninit};
[01:28:59] +   |                       ^^^^
[01:28:59]     |
[01:28:59]     |
[01:28:59] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59]  
[01:28:59] -error: reference to zeroed memory
[01:28:59] -error: reference to zeroed memory
[01:28:59] +error: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[01:28:59]     |
[01:28:59]     |
[01:28:59]  LL |     let ref_zero: &T = std::intrinsics::init(); // warning
[01:28:59] -   |
[01:28:59] -   |
[01:28:59] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59]  
[01:28:59] -error: reference to uninitialized memory
[01:28:59] -  --> $DIR/invalid_ref.rs:36:26
[01:28:59] -  --> $DIR/invalid_ref.rs:36:26
[01:28:59] +error: use of deprecated item 'std::intrinsics::init': no longer used by rustc, will be removed - use MaybeUnint instead
[01:28:59]     |
[01:28:59] -LL |     let ref_uninit: &T = std::mem::uninitialized(); // warning
[01:28:59] -   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:28:59] -   |
[01:28:59] -   |
[01:28:59] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59] +LL |     let mem_zero: usize = std::intrinsics::init(); // no warning
[01:28:59]  
[01:28:59] -error: reference to uninitialized memory
[01:28:59] -  --> $DIR/invalid_ref.rs:40:26
[01:28:59] -   |
[01:28:59] -   |
[01:28:59] -LL |     let ref_uninit: &T = core::mem::uninitialized(); // warning
[01:28:59] -   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:28:59] -   |
[01:28:59] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59]  
[01:28:59] -error: reference to uninitialized memory
[01:28:59] -  --> $DIR/invalid_ref.rs:44:26
[01:28:59] -   |
[01:28:59] -   |
[01:28:59] -LL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning
[01:28:59] -   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:28:59] -   |
[01:28:59] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:28:59] -error: aborting due to 6 previous errors
[01:28:59] -
[01:28:59] +Some errors have detailed explanations: E0425, E0432.
[01:28:59] +For more information about an error, try `rustc --explain E0425`.
---
[01:28:59] 
[01:28:59] ------------------------------------------
[01:28:59] stderr:
[01:28:59] ------------------------------------------
[01:28:59] {"message":"unresolved import `std::intrinsics::uninit`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n