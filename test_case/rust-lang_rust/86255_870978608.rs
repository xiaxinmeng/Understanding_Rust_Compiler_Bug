plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0599]: no method named `sess` found for struct `rustc_middle::ty::TyCtxt<'tcx>` in the current scope
  --> src/vtable.rs:77:30
   |
77 |             Err(_) => fx.tcx.sess().fatal("allocation of constant vtable failed"),
   |                              ^^^^-- help: remove the arguments
   |                              field, not a method
   | 
   | 
  ::: /checkout/compiler/rustc_query_system/src/dep_graph/mod.rs:36:8
   |
36 |     fn sess(&self) -> &Session;
   |        ---- the method is available for `rustc_middle::ty::TyCtxt<'tcx>` here
   = help: items from traits can only be used if the trait is in scope
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
           `use crate::rustc_middle::dep_graph::DepContext;`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_codegen_cranelift`
