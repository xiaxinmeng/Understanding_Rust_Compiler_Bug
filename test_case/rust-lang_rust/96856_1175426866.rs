plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: unexpected closing delimiter: `}`
   --> compiler/rustc_const_eval/src/transform/validate.rs:900:1
    |
889 |     fn visit_source_scope(&mut self, scope: SourceScope) {
    |                                                          - this opening brace...
899 |     }
899 |     }
    |     - ...matches this closing brace
    | ^ unexpected closing delimiter

error: mismatched closing delimiter: `)`
   --> compiler/rustc_const_eval/src/transform/validate.rs:280:51
   --> compiler/rustc_const_eval/src/transform/validate.rs:280:51
    |
280 |                     ty::Opaque(def_id, substs) => {
    |                                                   ^ unclosed delimiter
281 |                         self.tcx.bound_type_of(def_id)).subst(self.tcx, substs).kind()

error: could not compile `rustc_const_eval` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_const_eval` due to 2 previous errors
