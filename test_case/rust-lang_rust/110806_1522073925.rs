plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `mk_trait_ref` found for struct `rustc_middle::ty::TyCtxt<'tcx>` in the current scope
  --> src/librustdoc/clean/auto_trait.rs:47:47
   |
47 |         let trait_ref = ty::Binder::dummy(tcx.mk_trait_ref(trait_def_id, [ty]));
   |                                               ^^^^^^^^^^^^ help: there is a method with a similar name: `mk_ref`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:01:16
