plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0433]: failed to resolve: use of undeclared type `FxIndexSet`
   --> compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs:215:44
    |
215 |             let mut v = TraitObjectVisitor(FxIndexSet::default());
    |
help: consider importing one of these items
    |
    |
3   | use crate::infer::error_reporting::FxIndexSet;
    |
3   | use rustc_data_structures::fx::FxIndexSet;


error[E0412]: cannot find type `FxIndexSet` in this scope
   --> compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs:383:25
    |
383 |         trait_objects: &FxIndexSet<DefId>,
    |
help: consider importing one of these items
    |
    |
3   | use crate::infer::error_reporting::FxIndexSet;
    |
3   | use rustc_data_structures::fx::FxIndexSet;


error[E0433]: failed to resolve: use of undeclared type `FxIndexSet`
   --> compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs:465:40
    |
465 |         let mut v = TraitObjectVisitor(FxIndexSet::default());
    |
help: consider importing one of these items
    |
    |
3   | use crate::infer::error_reporting::FxIndexSet;
    |
3   | use rustc_data_structures::fx::FxIndexSet;


error[E0412]: cannot find type `FxIndexSet` in this scope
   --> compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs:481:22
    |
481 |         found_dids: &FxIndexSet<DefId>,
    |
help: consider importing one of these items
    |
    |
3   | use crate::infer::error_reporting::FxIndexSet;
    |
3   | use rustc_data_structures::fx::FxIndexSet;


error[E0412]: cannot find type `FxIndexSet` in this scope
   --> compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs:501:35
    |
501 | pub struct TraitObjectVisitor(pub FxIndexSet<DefId>);
    |
help: consider importing one of these items
    |
    |
3   | use crate::infer::error_reporting::FxIndexSet;
    |
3   | use rustc_data_structures::fx::FxIndexSet;


error: unused import: `rustc_data_structures::fx::FxHashSet`
  --> compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs:11:5
   |
11 | use rustc_data_structures::fx::FxHashSet;
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rustc_infer` due to 6 previous errors
