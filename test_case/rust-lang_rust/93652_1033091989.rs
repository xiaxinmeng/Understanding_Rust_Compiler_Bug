plain
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0433]: failed to resolve: use of undeclared crate or module `hir`
   --> compiler/rustc_trait_selection/src/traits/coherence.rs:339:28
    |
339 |         let body_id = hir::HirId::make_owner(impl1_def_id);
    |                            ^^^^^ not found in `hir`
help: consider importing one of these items
    |
    |
7   | use crate::traits::hir::HirId;
    |
7   | use rustc_hir::HirId;

error[E0425]: cannot find value `item_id` in this scope
   --> compiler/rustc_trait_selection/src/traits/coherence.rs:438:46
    |
    |
438 |             outlives_env.save_implied_bounds(item_id);

    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
Some errors have detailed explanations: E0425, E0433.
For more information about an error, try `rustc --explain E0425`.
