plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/compare_method.rs:1516:35
     |
1516 |                     tcx.mk_opaque(key.def_id, key.substs),
     |                         --------- ^^^^^^^^^^ expected struct `DefId`, found struct `LocalDefId`
     |                         arguments to this function are incorrect
     |
note: return type inferred to be `DefId` here
    --> compiler/rustc_typeck/src/check/compare_method.rs:1490:20
    --> compiler/rustc_typeck/src/check/compare_method.rs:1490:20
     |
1490 |             return Err(reported);
     |                    ^^^^^^^^^^^^^
note: associated function defined here
    --> /checkout/compiler/rustc_middle/src/ty/context.rs:2632:12
     |
2632 |     pub fn mk_opaque(self, def_id: DefId, substs: SubstsRef<'tcx>) -> Ty<'tcx> {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
