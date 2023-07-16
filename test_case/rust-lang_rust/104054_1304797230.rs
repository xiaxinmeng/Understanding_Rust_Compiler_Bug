plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: mismatched types
   --> src/constant.rs:401:13
    |
401 |         for &(offset, alloc_id) in alloc.provenance().iter() {
    |             ^^^^^^^^^^^^^^^^^^^    ------------------------- this is an iterator with items of type `AllocId`
    |             expected struct `AllocId`, found reference
    |
    = note: expected struct `AllocId`
            found reference `&_`
            found reference `&_`

error[E0277]: `Allocation` doesn't implement `std::fmt::Debug`
   --> src/constant.rs:434:29
    |
432 |                           tcx.sess.fatal(&format!(
    |  _________________________________________-
433 | |                             "Allocation {:?} contains reference to TLS value {:?}",
434 | |                             alloc, def_id
    | |                             ^^^^^ `Allocation` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
    | |_________________________- in this macro invocation (#1)
    |
   ::: /checkout/library/alloc/src/macros.rs:116:1
    |
