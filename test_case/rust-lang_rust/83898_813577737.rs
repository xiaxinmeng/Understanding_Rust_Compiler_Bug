plain
    Checking chalk-engine v0.55.0
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `Rc<ObligationCauseData<'tcx>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/mod.rs:1592:14
     |
1592 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
     |              ^^^^^^^^ --------------------------------------------------- within this `[closure@compiler/rustc_middle/src/ty/mod.rs:1592:23: 1592:74]`
     |              |
     |              `Rc<ObligationCauseData<'tcx>>` cannot be shared between threads safely
     |
     = help: within `[closure@compiler/rustc_middle/src/ty/mod.rs:1592:23: 1592:74]`, the trait `std::marker::Sync` is not implemented for `Rc<ObligationCauseData<'tcx>>`
     = note: required because it appears within the type `Option<Rc<ObligationCauseData<'tcx>>>`
     = note: required because it appears within the type `ObligationCause<'tcx>`
     = note: required because it appears within the type `Option<ObligationCause<'tcx>>`
     = note: required because it appears within the type `((Predicate<'tcx>, rustc_hir::HirId), Option<ObligationCause<'tcx>>)`
     = note: required because it appears within the type `std::marker::PhantomData<((Predicate<'tcx>, rustc_hir::HirId), Option<ObligationCause<'tcx>>)>`
     = note: required because it appears within the type `rustc_query_system::query::caches::DefaultCache<(Predicate<'tcx>, rustc_hir::HirId), Option<ObligationCause<'tcx>>>`
     = note: required because it appears within the type `rustc_query_system::query::QueryCacheStore<rustc_query_system::query::caches::DefaultCache<(Predicate<'tcx>, rustc_hir::HirId), Option<ObligationCause<'tcx>>>>`
     = note: required because it appears within the type `QueryCaches<'tcx>`
     = note: required because it appears within the type `GlobalCtxt<'tcx>`
     = note: required because it appears within the type `&'tcx GlobalCtxt<'tcx>`
     = note: required because it appears within the type `context::TyCtxt<'tcx>`
     = note: required because it appears within the type `&context::TyCtxt<'tcx>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/ty/mod.rs:1592:23: 1592:74]`

error[E0277]: `Rc<ObligationCauseData<'tcx>>` cannot be sent between threads safely
    --> compiler/rustc_middle/src/ty/mod.rs:1592:14
     |
1592 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
     |              ^^^^^^^^ `Rc<ObligationCauseData<'tcx>>` cannot be sent between threads safely
     |
     = help: within `((Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex))`, the trait `Send` is not implemented for `Rc<ObligationCauseData<'tcx>>`
     = note: required because it appears within the type `Option<Rc<ObligationCauseData<'tcx>>>`
     = note: required because it appears within the type `ObligationCause<'tcx>`
     = note: required because it appears within the type `Option<ObligationCause<'tcx>>`
     = note: required because it appears within the type `(Option<ObligationCause<'tcx>>, DepNodeIndex)`
     = note: required because it appears within the type `((Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex))`
     = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<((Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex))>`
     = note: required because it appears within the type `hashbrown::map::HashMap<(Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<(Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, HashMap<(Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::Lock<HashMap<(Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>`
     = note: required because it appears within the type `[sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>; 32]`
     = note: required because it appears within the type `Sharded<HashMap<(Predicate<'tcx>, rustc_hir::HirId), (Option<ObligationCause<'tcx>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_query_system::query::QueryCacheStore<rustc_query_system::query::caches::DefaultCache<(Predicate<'tcx>, rustc_hir::HirId), Option<ObligationCause<'tcx>>>>`
     = note: required because it appears within the type `QueryCaches<'tcx>`
     = note: required because it appears within the type `GlobalCtxt<'tcx>`
     = note: required because it appears within the type `&'tcx GlobalCtxt<'tcx>`
     = note: required because it appears within the type `context::TyCtxt<'tcx>`
     = note: required because it appears within the type `&context::TyCtxt<'tcx>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/ty/mod.rs:1592:23: 1592:74]`

error[E0277]: `Rc<ObligationCauseData<'_>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/context.rs:1748:13
     |
1748 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Rc<ObligationCauseData<'_>>` cannot be shared between threads safely
    ::: /checkout/compiler/rustc_data_structures/src/sync.rs:420:32
     |
     |
420  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ---- required by this bound in `assert_sync`
     |
     = help: within `ImplicitCtxt<'_, '_>`, the trait `std::marker::Sync` is not implemented for `Rc<ObligationCauseData<'_>>`
     = note: required because it appears within the type `Option<Rc<ObligationCauseData<'_>>>`
     = note: required because it appears within the type `ObligationCause<'_>`
     = note: required because it appears within the type `Option<ObligationCause<'_>>`
     = note: required because it appears within the type `((Predicate<'_>, rustc_hir::HirId), Option<ObligationCause<'_>>)`
     = note: required because it appears within the type `std::marker::PhantomData<((Predicate<'_>, rustc_hir::HirId), Option<ObligationCause<'_>>)>`
     = note: required because it appears within the type `rustc_query_system::query::caches::DefaultCache<(Predicate<'_>, rustc_hir::HirId), Option<ObligationCause<'_>>>`
     = note: required because it appears within the type `rustc_query_system::query::QueryCacheStore<rustc_query_system::query::caches::DefaultCache<(Predicate<'_>, rustc_hir::HirId), Option<ObligationCause<'_>>>>`
     = note: required because it appears within the type `QueryCaches<'_>`
     = note: required because it appears within the type `GlobalCtxt<'_>`
     = note: required because it appears within the type `&GlobalCtxt<'_>`
     = note: required because it appears within the type `context::TyCtxt<'_>`
     = note: required because it appears within the type `ImplicitCtxt<'_, '_>`

error[E0277]: `Rc<ObligationCauseData<'_>>` cannot be sent between threads safely
    --> compiler/rustc_middle/src/ty/context.rs:1748:13
     |
1748 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Rc<ObligationCauseData<'_>>` cannot be sent between threads safely
    ::: /checkout/compiler/rustc_data_structures/src/sync.rs:420:32
     |
     |
420  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ---- required by this bound in `assert_sync`
     |
     = help: within `((Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex))`, the trait `Send` is not implemented for `Rc<ObligationCauseData<'_>>`
     = note: required because it appears within the type `Option<Rc<ObligationCauseData<'_>>>`
     = note: required because it appears within the type `ObligationCause<'_>`
     = note: required because it appears within the type `Option<ObligationCause<'_>>`
     = note: required because it appears within the type `(Option<ObligationCause<'_>>, DepNodeIndex)`
     = note: required because it appears within the type `((Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex))`
     = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<((Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex))>`
     = note: required because it appears within the type `hashbrown::map::HashMap<(Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<(Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, HashMap<(Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::Lock<HashMap<(Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>`
     = note: required because it appears within the type `[sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>; 32]`
     = note: required because it appears within the type `Sharded<HashMap<(Predicate<'_>, rustc_hir::HirId), (Option<ObligationCause<'_>>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_query_system::query::QueryCacheStore<rustc_query_system::query::caches::DefaultCache<(Predicate<'_>, rustc_hir::HirId), Option<ObligationCause<'_>>>>`
     = note: required because it appears within the type `QueryCaches<'_>`
     = note: required because it appears within the type `GlobalCtxt<'_>`
     = note: required because it appears within the type `&GlobalCtxt<'_>`
     = note: required because it appears within the type `context::TyCtxt<'_>`
     = note: required because it appears within the type `ImplicitCtxt<'_, '_>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle`
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_lint_defs" "-p" "rustc_serialize" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_fs_util" "-p" "rustc_index" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_data_structures" "-p" "rustc_symbol_mangling" "-p" "rustc_target" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_apfloat" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_span" "-p" "rustc_macros" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_query_impl" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_expand" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_infer" "-p" "rustc_privacy" "-p" "rustc_trait_selection" "-p" "rustc_ty_utils" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_passes" "-p" "rustc_passes" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_hir_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_metadata" "-p" "rustc_plugin_impl" "-p" "rustc_mir_build" "-p" "rustc_parse" "-p" "rustc_typeck" "-p" "rustc_save_analysis" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:23
