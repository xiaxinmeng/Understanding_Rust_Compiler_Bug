plain
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: call to `.deref()` on a reference in this situation does nothing
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:3448:95
     |
3448 |             if let ObligationCauseCode::ExprBindingObligation(def_id, _, _, idx) = parent_code.deref()
     |                                                                                               ^^^^^^^^ unnecessary method call
     |
     = note: the type `&rustc_middle::traits::ObligationCauseCode<'_>` which `deref` is being called on is the same as the type returned from `deref`, so the method call does not do anything and can be removed
     = note: `-D noop-method-call` implied by `-D warnings`
error: could not compile `rustc_trait_selection` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:06:38
cat: /tmp/toolstate/toolstates.json: No such file or directory
