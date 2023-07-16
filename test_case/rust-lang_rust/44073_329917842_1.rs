
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
   --> src/librustc/infer/mod.rs:501:14
    |
501 |         self.infer_ctxt().enter(|infcx| {
    |              ^^^^^^^^^^
    |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the impl at 473:1...
   --> src/librustc/infer/mod.rs:473:1
    |
473 | / impl<'a, 'gcx, 'tcx> TyCtxt<'a, 'gcx, 'tcx> {
474 | |     /// Currently, higher-ranked type bounds inhibit normalization. Therefore,
475 | |     /// each time we erase them in translation, we need to normalize
476 | |     /// the contents.
...   |
526 | |     }
527 | | }
    | |_^
note: ...so that types are compatible (expected ty::context::TyCtxt<'_, '_, '_>, found ty::context::TyCtxt<'a, 'gcx, 'tcx>)
   --> src/librustc/infer/mod.rs:501:14
    |
501 |         self.infer_ctxt().enter(|infcx| {
    |              ^^^^^^^^^^
note: but, the lifetime must be valid for the lifetime 'tcx as defined on the impl at 473:1...
   --> src/librustc/infer/mod.rs:473:1
    |
473 | / impl<'a, 'gcx, 'tcx> TyCtxt<'a, 'gcx, 'tcx> {
474 | |     /// Currently, higher-ranked type bounds inhibit normalization. Therefore,
475 | |     /// each time we erase them in translation, we need to normalize
476 | |     /// the contents.
...   |
526 | |     }
527 | | }
    | |_^
note: ...so that expression is assignable (expected &infer::InferCtxt<'_, '_, 'tcx>, found &infer::InferCtxt<'_, '_, '_>)
   --> src/librustc/infer/mod.rs:502:35
    |
502 |             value.trans_normalize(&infcx, param_env)
    |                                   ^^^^^^
