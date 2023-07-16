shell
error[E0425]: cannot find function `atomic_xchg_acq` in module `core::intrinsics`
   --> src/atomics.rs:167:33
    |
167 |         ) = ::core::intrinsics::atomic_xchg_acq(&mut x, 33 as libc::c_int);
    |                                 ^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_cxchg_acq`
    |
   ::: /home/kkysen/.rustup/toolchains/nightly-2022-08-08-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:320:5
    |
320 |     pub fn atomic_cxchg_acquire_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    |     -------------------------------------------------------------------------------------- similarly named function `atomic_cxchg_acq` defined here

error[E0425]: cannot find function `atomic_store_rel` in module `core::intrinsics`
   --> src/atomics.rs:171:25
    |
171 |     ::core::intrinsics::atomic_store_rel(&mut x, 0);
    |                         ^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `atomic_store`
    |
   ::: /home/kkysen/.rustup/toolchains/nightly-2022-08-08-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:523:5
    |
523 |     pub fn atomic_store_seqcst<T: Copy>(dst: *mut T, val: T);
    |     -------------------------------------------------------- similarly named function `atomic_store` defined here
