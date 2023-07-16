plain
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
[RUSTC-TIMING] build_script_build test:false 0.273
[RUSTC-TIMING] build_script_build test:false 0.560
error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
   --> library/core/src/../../stdarch/crates/core_arch/src/riscv_shared/mod.rs:608:1
    |
608 | #[target_feature(enable = "zksh")]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
609 | pub fn sm3p0(x: u32) -> u32 {
    | --------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
   --> library/core/src/../../stdarch/crates/core_arch/src/riscv_shared/mod.rs:638:1
    |
638 | #[target_feature(enable = "zksh")]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
639 | pub fn sm3p1(x: u32) -> u32 {
    | --------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
   --> library/core/src/../../stdarch/crates/core_arch/src/riscv_shared/mod.rs:692:1
    |
692 | #[target_feature(enable = "zksed")]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
693 | pub fn sm4ed<const BS: u8>(x: u32, a: u32) -> u32 {
    | ------------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
   --> library/core/src/../../stdarch/crates/core_arch/src/riscv_shared/mod.rs:752:1
    |
752 | #[target_feature(enable = "zksed")]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
753 | pub fn sm4ks<const BS: u8>(x: u32, k: u32) -> u32 {
    | ------------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
[RUSTC-TIMING] core test:false 9.166
error: could not compile `core` due to 4 previous errors
Build completed unsuccessfully in 0:05:44
