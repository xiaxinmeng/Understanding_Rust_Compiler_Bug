
[00:24:45] warning: `#[must_use]` on methods is experimental (see issue #43302)
[00:24:45]    --> libcore/../stdsimd/coresimd/ppsv/api/minimal.rs:76:13
[00:24:45]     |
[00:24:45] 76  |               #[must_use = "replace_unchecked does not modify the original value - it returns a new vector with the value at `index` replaced by `new_value`d"]
[00:24:45]     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:24:45]     | 
[00:24:45]    ::: libcore/../stdsimd/coresimd/ppsv/v512.rs:124:1
[00:24:45]     |
[00:24:45] 124 | / simd_f_ty! {
[00:24:45] 125 | |     f64x8: 8, f64, b8x8, f64x8_tests |
[00:24:45] 126 | |     f64, f64, f64, f64, f64, f64, f64, f64 |
[00:24:45] 127 | |     x0, x1, x2, x3, x4, x5, x6, x7 |
[00:24:45] 128 | |     /// A 512-bit vector with 8 `f64` lanes.
[00:24:45] 129 | | }
[00:24:45]     | |_- in this macro invocation
[00:24:45]     |
[00:24:45]     = help: add #![feature(fn_must_use)] to the crate attributes to enable
[00:24:45] 
[00:24:45] error[E0658]: #[doc(cfg(...))] is experimental (see issue #43781)
[00:24:45]   --> libcore/../stdsimd/coresimd/mod.rs:52:5
[00:24:45]    |
[00:24:45] 52 |     #[doc(cfg(target_arch = "x86_64"))]
[00:24:45]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:24:45]    |
[00:24:45]    = help: add #![feature(doc_cfg)] to the crate attributes to enable
[00:24:45] 
[00:24:45]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:24:45] error: aborting due to previous error
