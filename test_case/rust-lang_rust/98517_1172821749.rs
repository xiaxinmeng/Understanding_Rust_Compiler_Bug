plain
   Compiling object v0.26.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling hashbrown v0.12.0
   Compiling addr2line v0.16.0
error[E0432]: unresolved imports `core::intrinsics::atomic_store_rel`, `core::intrinsics::atomic_xsub_rel`
   |
   |
11 | use core::intrinsics::{atomic_store_rel, atomic_xsub_rel};
   |                        ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^ no `atomic_xsub_rel` in `intrinsics`
   |                        |
   |                        no `atomic_store_rel` in `intrinsics`
help: a similar name exists in the module
   |
   |
11 | use core::intrinsics::{atomic_store_release, atomic_xsub_rel};
help: a similar name exists in the module
   |
   |
11 | use core::intrinsics::{atomic_store_rel, atomic_xsub_acqrel};

For more information about this error, try `rustc --explain E0432`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:05:00
