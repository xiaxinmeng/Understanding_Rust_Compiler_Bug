plain
   Compiling panic_unwind v0.0.0 (D:\a\rust\rust\library\panic_unwind)
   Compiling gimli v0.25.0
   Compiling object v0.26.2
   Compiling miniz_oxide v0.4.0
error[E0432]: unresolved import `core::intrinsics::atomic_store`
   --> library\panic_unwind\src\seh.rs:259:9
259 |     use core::intrinsics::atomic_store;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `atomic_store` in `intrinsics`

[RUSTC-TIMING] rustc_std_workspace_alloc test:false 0.044
---
[RUSTC-TIMING] core test:false 35.137
[RUSTC-TIMING] gimli test:false 7.250
[RUSTC-TIMING] object test:false 7.250
Build completed unsuccessfully in 0:13:02
make: *** [Makefile:72: ci-subset-2] Error 1
