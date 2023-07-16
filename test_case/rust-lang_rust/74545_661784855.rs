
error: `std::intrinsics::transmute` is not yet stable as a const fn
   --> /home/tiong/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_ast-670.0.0/ptr.rs:136:17
    |
136 |                 std::mem::transmute(NonNull::<[T; 0]>::dangling() as NonNull<[T]>)
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add `#![feature(const_transmute)]` to the crate attributes to enable

