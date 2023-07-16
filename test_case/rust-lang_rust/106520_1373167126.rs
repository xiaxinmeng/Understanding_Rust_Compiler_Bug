plain
   Compiling proc-macro-error v1.0.4
error[E0725]: the feature `rustc_attrs` is not in the list of allowed features
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/lib.rs:99:34
   |
99 | #![cfg_attr(rustc_attrs, feature(rustc_attrs))]

error[E0725]: the feature `core_intrinsics` is not in the list of allowed features
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/lib.rs:116:13
    |
    |
116 |     feature(core_intrinsics)
    |             ^^^^^^^^^^^^^^^

error[E0658]: the `#[rustc_layout_scalar_valid_range_start]` attribute is just used to enable niche optimizations in libcore and libstd and will never be stable
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:28:25
   |
28 | #[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_start(0xf001))]
   |
   = help: add `#![feature(rustc_attrs)]` to the crate attributes to enable


error[E0658]: the `#[rustc_layout_scalar_valid_range_end]` attribute is just used to enable niche optimizations in libcore and libstd and will never be stable
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:29:25
   |
29 | #[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_end(0xffff))]
   |
   = help: add `#![feature(rustc_attrs)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:171:13
    |
171 |             core::intrinsics::assume(raw.is_in_range(-4095..0));
    |
    = help: add `#![feature(core_intrinsics)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustix-0.36.5/src/backend/linux_raw/io/errno.rs:200:13
    |
200 |             core::intrinsics::assume(raw.is_in_range(-4095..0));
    |
    = help: add `#![feature(core_intrinsics)]` to the crate attributes to enable

Some errors have detailed explanations: E0658, E0725.
