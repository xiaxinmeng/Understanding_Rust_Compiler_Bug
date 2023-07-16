plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0507]: cannot move out of an `Arc`
   --> compiler/rustc_codegen_ssa/src/back/link.rs:267:45
    |
267 |         let path = if let Some((path, _)) = used_crate_source.rlib {
    |                                 ----        ^^^^^^^^^^^^^^^^^^^^^^ help: consider borrowing here: `&used_crate_source.rlib`
    |                                 data moved here
    |                                 data moved here
    |                                 move occurs because `path` has type `PathBuf`, which does not implement the `Copy` trait
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
