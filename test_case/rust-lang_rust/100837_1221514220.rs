plain
[RUSTC-TIMING] gimli test:false 5.195
[RUSTC-TIMING] object test:false 5.323
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error[E0658]: use of unstable library feature 'pointer_is_aligned'
   --> library/std/src/sys/sgx/abi/usercalls/alloc.rs:120:53
    |
120 |         let is_aligned = |p: *const u8| -> bool { p.is_aligned_to(Self::align_of()) };
    |
    = note: see issue #96284 <https://github.com/rust-lang/rust/issues/96284> for more information
    = help: add `#![feature(pointer_is_aligned)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'pointer_is_aligned'
   --> library/std/src/sys/sgx/abi/usercalls/alloc.rs:389:35
    |
389 |     } else if len % 8 == 0 && dst.is_aligned_to(8) {
    |
    = note: see issue #96284 <https://github.com/rust-lang/rust/issues/96284> for more information
    = help: add `#![feature(pointer_is_aligned)]` to the crate attributes to enable

