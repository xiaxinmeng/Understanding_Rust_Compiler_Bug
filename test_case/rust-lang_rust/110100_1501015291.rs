plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0425]: cannot find value `infcx` in this scope
   --> compiler/rustc_trait_selection/src/traits/select/mod.rs:540:24
    |
540 |             let goal = infcx.resolve_vars_if_possible((obligation.predicate, obligation.param_env));
    |                        ^^^^^ help: you might have meant to use the available field: `self.infcx`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to previous error
