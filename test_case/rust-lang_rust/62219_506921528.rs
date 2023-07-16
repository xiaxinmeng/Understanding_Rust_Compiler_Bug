plain
[01:30:39] expected stderr:
[01:30:39] error: reference to zeroed memory
[01:30:39]   --> $DIR/invalid_ref.rs:24:24
[01:30:39]    |
[01:30:39] LL |     let ref_zero: &T = std::mem::zeroed(); // warning
[01:30:39]    |
[01:30:39]    = note: #[deny(clippy::invalid_ref)] on by default
[01:30:39]    = note: #[deny(clippy::invalid_ref)] on by default
[01:30:39]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] error: reference to zeroed memory
[01:30:39]   --> $DIR/invalid_ref.rs:28:24
[01:30:39]    |
[01:30:39]    |
[01:30:39] LL |     let ref_zero: &T = core::mem::zeroed(); // warning
[01:30:39]    |
[01:30:39]    |
[01:30:39]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] error: reference to zeroed memory
[01:30:39]   --> $DIR/invalid_ref.rs:32:24
[01:30:39]    |
[01:30:39]    |
[01:30:39] LL |     let ref_zero: &T = std::intrinsics::init(); // warning
[01:30:39]    |
[01:30:39]    |
[01:30:39]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] error: reference to uninitialized memory
[01:30:39]   --> $DIR/invalid_ref.rs:36:26
[01:30:39]    |
[01:30:39] LL |     let ref_uninit: &T = std::mem::uninitialized(); // warning
[01:30:39] LL |     let ref_uninit: &T = std::mem::uninitialized(); // warning
[01:30:39]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:30:39]    |
[01:30:39]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] error: reference to uninitialized memory
[01:30:39]   --> $DIR/invalid_ref.rs:40:26
[01:30:39]    |
[01:30:39] LL |     let ref_uninit: &T = core::mem::uninitialized(); // warning
[01:30:39] LL |     let ref_uninit: &T = core::mem::uninitialized(); // warning
[01:30:39]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:30:39]    |
[01:30:39]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] error: reference to uninitialized memory
[01:30:39]   --> $DIR/invalid_ref.rs:44:26
[01:30:39]    |
[01:30:39] LL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning
[01:30:39] LL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning
[01:30:39]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:30:39]    |
[01:30:39]    = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] error: aborting due to 6 previous errors
[01:30:39] 
[01:30:39] 
[01:30:39] 
[01:30:39] 
[01:30:39] diff of stderr:
[01:30:39] 
[01:30:39] -error: reference to zeroed memory
[01:30:39] -  --> $DIR/invalid_ref.rs:24:24
[01:30:39] +error[E0432]: unresolved import `std::intrinsics::uninit`
[01:30:39] +  --> $DIR/invalid_ref.rs:5:29
[01:30:39]     |
[01:30:39] -LL |     let ref_zero: &T = std::mem::zeroed(); // warning
[01:30:39] -   |
[01:30:39] -   = note: #[deny(clippy::invalid_ref)] on by default
[01:30:39] -   = note: #[deny(clippy::invalid_ref)] on by default
[01:30:39] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] +LL | use std::intrinsics::{init, uninit};
[01:30:39] +   |                             |
[01:30:39] +   |                             no `uninit` in `intrinsics`
[01:30:39] +   |                             help: a similar name exists in the module: `init`
[01:30:39]  
[01:30:39]  
[01:30:39] -error: reference to zeroed memory
[01:30:39] -  --> $DIR/invalid_ref.rs:28:24
[01:30:39] +error[E0425]: cannot find function `uninit` in module `std::intrinsics`
[01:30:39] +  --> $DIR/invalid_ref.rs:44:43
[01:30:39]     |
[01:30:39] -LL |     let ref_zero: &T = core::mem::zeroed(); // warning
[01:30:39] -   |
[01:30:39] -   |
[01:30:39] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] -error: reference to zeroed memory
[01:30:39] -  --> $DIR/invalid_ref.rs:32:24
[01:30:39] -   |
[01:30:39] -   |
[01:30:39] -LL |     let ref_zero: &T = std::intrinsics::init(); // warning
[01:30:39] -   |
[01:30:39] -   |
[01:30:39] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] -error: reference to uninitialized memory
[01:30:39] -  --> $DIR/invalid_ref.rs:36:26
[01:30:39] -   |
[01:30:39] -LL |     let ref_uninit: &T = std::mem::uninitialized(); // warning
[01:30:39] -LL |     let ref_uninit: &T = std::mem::uninitialized(); // warning
[01:30:39] -   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:30:39] -   |
[01:30:39] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] -error: reference to uninitialized memory
[01:30:39] -  --> $DIR/invalid_ref.rs:40:26
[01:30:39] -   |
[01:30:39] -LL |     let ref_uninit: &T = core::mem::uninitialized(); // warning
[01:30:39] -LL |     let ref_uninit: &T = core::mem::uninitialized(); // warning
[01:30:39] -   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:30:39] -   |
[01:30:39] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39] -error: reference to uninitialized memory
[01:30:39] -  --> $DIR/invalid_ref.rs:44:26
[01:30:39] -   |
[01:30:39]  LL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning
[01:30:39]  LL |     let ref_uninit: &T = std::intrinsics::uninit(); // warning
[01:30:39] -   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:30:39] -   |
[01:30:39] -   = help: Creation of a null reference is undefined behavior; see https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[01:30:39]  
[01:30:39] -error: aborting due to 6 previous errors
[01:30:39] +error: aborting due to 2 previous errors
[01:30:39]  
---
[01:30:39] 
[01:30:39] ------------------------------------------
[01:30:39] stderr:
[01:30:39] ------------------------------------------
[01:30:39] {"message":"unresolved import `std::intrinsics::uninit`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n