plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `sess` found for struct `context::TyCtxt<'tcx>` in the current scope
    --> compiler/rustc_middle/src/values.rs:46:17
     |
46   |             tcx.sess().abort_if_errors();
     |                 ^^^^ field, not a method
    ::: compiler/rustc_middle/src/ty/context.rs:1059:1
     |
1059 | pub struct TyCtxt<'tcx> {
1059 | pub struct TyCtxt<'tcx> {
     | ----------------------- method `sess` not found for this struct
    ::: /checkout/compiler/rustc_query_system/src/dep_graph/mod.rs:35:8
     |
35   |     fn sess(&self) -> &Session;
     |        ---- the method is available for `context::TyCtxt<'tcx>` here
     |        ---- the method is available for `context::TyCtxt<'tcx>` here
     |
     = help: items from traits can only be used if the trait is in scope
help: remove the arguments
     |
46   -             tcx.sess().abort_if_errors();
46   +             tcx.sess.abort_if_errors();
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
1    | use rustc_query_system::dep_graph::DepContext;
     |
