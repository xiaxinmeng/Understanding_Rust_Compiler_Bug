plain
...............................................i........................................  2376/14920
..............F.........................................................................  2464/14920
........................................................................................  2552/14920
........................................................................................  2640/14920
....................................................................F.....F.............  2728/14920
........................................................................................  2904/14920
........................................................................................  2992/14920
........................................................................................  3080/14920
........................................................................................  3168/14920
---
diff of stderr:

8   --> $DIR/const-arg-in-const-arg.rs:24:14
9    |
10 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
-    |              ^^query stack during panic:
+ query stack during panic:
+ query stack during panic:
12 #0 [mir_borrowck] borrow-checking `test::{constant#3}`
13 #1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{constant#3}`
14 #2 [mir_for_ctfe] caching mir of `test::{constant#3}` for CTFE

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.full/const-arg-in-const-arg.full.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const-arg-in-const-arg.rs`

error in revision `full`: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/const-arg-in-const-arg.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.full/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0794]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:32:23
   |
LL |     let _: [u8; faz::<'a>(&())]; //[min]~ ERROR generic parameters may not
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:24:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }


error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:882:36: cannot convert `ReFree(DefId(0:14 ~ const_arg_in_const_arg[41ab]::test), BrNamed(DefId(0:15 ~ const_arg_in_const_arg[41ab]::test::'a), 'a))` to a region vid
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1643:9
stack backtrace:
   0:     0x7fabdb3c34b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::haa8a3bcced70ba70
   1:     0x7fabdb42a8e8 - core::fmt::write::hbf4502135b21ec09
   1:     0x7fabdb42a8e8 - core::fmt::write::hbf4502135b21ec09
   2:     0x7fabdb3b7b21 - std::io::Write::write_fmt::h7d9967ab0bf5a25f
   3:     0x7fabdb3c32c1 - std::sys_common::backtrace::print::h027b8b584a5f69d4
   4:     0x7fabdb3c644a - std::panicking::default_hook::{{closure}}::h9c5d941200c29b74
   5:     0x7fabdb3c612c - std::panicking::default_hook::h83fba6cfc1469036
   6:     0x7fabdbe89665 - rustc_driver_impl[6cef5cb93700381e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fabdb3c6b57 - std::panicking::rust_panic_with_hook::h2a1d9d43d66bf9d8
   8:     0x7fabdeb23e73 - std[9214d4328e246da4]::panicking::begin_panic::<rustc_errors[cc66173c7dba2577]::ExplicitBug>::{closure#0}
   9:     0x7fabdeb1dcb6 - std[9214d4328e246da4]::sys_common::backtrace::__rust_end_short_backtrace::<std[9214d4328e246da4]::panicking::begin_panic<rustc_errors[cc66173c7dba2577]::ExplicitBug>::{closure#0}, !>
  10:     0x7fabdbe21fb6 - std[9214d4328e246da4]::panicking::begin_panic::<rustc_errors[cc66173c7dba2577]::ExplicitBug>
  11:     0x7fabdead78c7 - <rustc_errors[cc66173c7dba2577]::HandlerInner>::bug::<alloc[7d7d4a2996a9152a]::string::String>
  12:     0x7fabdead6959 - <rustc_errors[cc66173c7dba2577]::Handler>::bug::<alloc[7d7d4a2996a9152a]::string::String>
  13:     0x7fabdec84737 - rustc_middle[7a2979d2db888602]::util::bug::opt_span_bug_fmt::<rustc_span[8be25b5ff532beb9]::span_encoding::Span>::{closure#0}
  14:     0x7fabdec8367c - rustc_middle[7a2979d2db888602]::ty::context::tls::with_opt::<rustc_middle[7a2979d2db888602]::util::bug::opt_span_bug_fmt<rustc_span[8be25b5ff532beb9]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7fabdec83604 - rustc_middle[7a2979d2db888602]::ty::context::tls::with_context_opt::<rustc_middle[7a2979d2db888602]::ty::context::tls::with_opt<rustc_middle[7a2979d2db888602]::util::bug::opt_span_bug_fmt<rustc_span[8be25b5ff532beb9]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7fabdbe2c622 - rustc_middle[7a2979d2db888602]::util::bug::bug_fmt
  17:     0x7fabdd3783f8 - <rustc_borrowck[5cf1a4439ffa6c37]::universal_regions::UniversalRegionIndices>::to_region_vid
  18:     0x7fabdd57cdd0 - <rustc_borrowck[5cf1a4439ffa6c37]::type_check::constraint_conversion::ConstraintConversion>::convert
  19:     0x7fabdd57ec3d - <rustc_borrowck[5cf1a4439ffa6c37]::type_check::constraint_conversion::ConstraintConversion>::convert_all
  20:     0x7fabdd3f1724 - <rustc_borrowck[5cf1a4439ffa6c37]::type_check::TypeChecker>::push_region_constraints
  21:     0x7fabdd3ed2ea - <rustc_borrowck[5cf1a4439ffa6c37]::type_check::TypeChecker>::ascribe_user_type
  22:     0x7fabdd3d7229 - rustc_borrowck[5cf1a4439ffa6c37]::type_check::type_check
  23:     0x7fabdd42ce29 - rustc_borrowck[5cf1a4439ffa6c37]::nll::compute_regions
  24:     0x7fabdd2919f5 - rustc_borrowck[5cf1a4439ffa6c37]::do_mir_borrowck
  25:     0x7fabdd27cb90 - rustc_borrowck[5cf1a4439ffa6c37]::mir_borrowck
  26:     0x7fabddb13856 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::mir_borrowck, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  27:     0x7fabdd95285f - rustc_query_impl[39a2c1aa0c45d430]::get_query::mir_borrowck
  28:     0x7fabdc89b129 - rustc_middle[7a2979d2db888602]::ty::query::query_get_at::<rustc_query_system[ef233bdbbc366e72]::query::caches::VecCache<rustc_span[8be25b5ff532beb9]::def_id::LocalDefId, rustc_middle[7a2979d2db888602]::query::erase::Erased<[u8; 8usize]>>>
  29:     0x7fabdc8d2477 - rustc_mir_transform[7a71ad9bda758d2e]::mir_drops_elaborated_and_const_checked
  30:     0x7fabddc3d04e - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  31:     0x7fabdd93614f - rustc_query_impl[39a2c1aa0c45d430]::get_query::mir_drops_elaborated_and_const_checked
  32:     0x7fabdc89b129 - rustc_middle[7a2979d2db888602]::ty::query::query_get_at::<rustc_query_system[ef233bdbbc366e72]::query::caches::VecCache<rustc_span[8be25b5ff532beb9]::def_id::LocalDefId, rustc_middle[7a2979d2db888602]::query::erase::Erased<[u8; 8usize]>>>
  33:     0x7fabdc8d219d - rustc_mir_transform[7a71ad9bda758d2e]::mir_for_ctfe
  34:     0x7fabddb14f46 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::mir_for_ctfe, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  35:     0x7fabdd93680e - rustc_query_impl[39a2c1aa0c45d430]::get_query::mir_for_ctfe
  36:     0x7fabdcc077ed - rustc_middle[7a2979d2db888602]::ty::query::query_get_at::<rustc_query_system[ef233bdbbc366e72]::query::caches::DefaultCache<rustc_span[8be25b5ff532beb9]::def_id::DefId, rustc_middle[7a2979d2db888602]::query::erase::Erased<[u8; 8usize]>>>
  37:     0x7fabdcc1147d - <rustc_const_eval[9b7277904ecbb64d]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[9b7277904ecbb64d]::interpret::machine::Machine>::load_mir
  38:     0x7fabdcb03d7c - <rustc_const_eval[9b7277904ecbb64d]::interpret::eval_context::InterpCx<rustc_const_eval[9b7277904ecbb64d]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  39:     0x7fabdcc56db6 - rustc_const_eval[9b7277904ecbb64d]::const_eval::eval_queries::eval_to_allocation_raw_provider
  40:     0x7fabddbd9182 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::eval_to_allocation_raw, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  41:     0x7fabdd9550b9 - rustc_query_impl[39a2c1aa0c45d430]::get_query::eval_to_allocation_raw
  42:     0x7fabdcb70b14 - <rustc_const_eval[9b7277904ecbb64d]::provide::{closure#0} as core[694345b096264287]::ops::function::FnOnce<(rustc_middle[7a2979d2db888602]::ty::context::TyCtxt, rustc_middle[7a2979d2db888602]::ty::ParamEnvAnd<rustc_middle[7a2979d2db888602]::mir::interpret::GlobalId>)>>::call_once
  43:     0x7fabddb44ef0 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::eval_to_valtree, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  44:     0x7fabdd955ed9 - rustc_query_impl[39a2c1aa0c45d430]::get_query::eval_to_valtree
  45:     0x7fabdea36ede - rustc_middle[7a2979d2db888602]::ty::query::query_get_at::<rustc_query_system[ef233bdbbc366e72]::query::caches::DefaultCache<rustc_middle[7a2979d2db888602]::ty::ParamEnvAnd<rustc_middle[7a2979d2db888602]::mir::interpret::GlobalId>, rustc_middle[7a2979d2db888602]::query::erase::Erased<[u8; 24usize]>>>
  46:     0x7fabdea526fd - <rustc_middle[7a2979d2db888602]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  47:     0x7fabdea51640 - <rustc_middle[7a2979d2db888602]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  48:     0x7fabde9f1522 - <rustc_infer[3c467000369b7c4b]::infer::InferCtxt>::const_eval_resolve
  49:     0x7fabde5e0122 - rustc_trait_selection[fb5980b19f5b378b]::traits::const_evaluatable::is_const_evaluatable
  50:     0x7fabde650418 - <rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::FulfillProcessor as rustc_data_structures[362fcaf54b071d0a]::obligation_forest::ObligationProcessor>::process_obligation
  51:     0x7fabde74c0d8 - <rustc_data_structures[362fcaf54b071d0a]::obligation_forest::ObligationForest<rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::FulfillProcessor>
  52:     0x7fabde6487b4 - <rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::FulfillmentContext as rustc_infer[3c467000369b7c4b]::traits::engine::TraitEngine>::select_where_possible
  53:     0x7fabdc36e9d2 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  54:     0x7fabdc3c7ce2 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::demand_coerce_diag
  55:     0x7fabdc358b86 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::demand_coerce
  56:     0x7fabdc3822f2 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_decl
  57:     0x7fabdc38260e - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_stmt
  58:     0x7fabdc382d05 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_block_with_expected
  59:     0x7fabdc3c963b - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_kind
  60:     0x7fabdc361604 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  61:     0x7fabdc3c8482 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  62:     0x7fabdc3634ad - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_return_expr
  63:     0x7fabdc547212 - rustc_hir_typeck[59f56756254854d1]::check::check_fn
  64:     0x7fabdc4336dc - rustc_hir_typeck[59f56756254854d1]::typeck
  65:     0x7fabddc4c106 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::typeck, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  66:     0x7fabdd9506ff - rustc_query_impl[39a2c1aa0c45d430]::get_query::typeck
  67:     0x7fabdc423879 - rustc_middle[7a2979d2db888602]::ty::query::query_get_at::<rustc_query_system[ef233bdbbc366e72]::query::caches::VecCache<rustc_span[8be25b5ff532beb9]::def_id::LocalDefId, rustc_middle[7a2979d2db888602]::query::erase::Erased<[u8; 8usize]>>>
  68:     0x7fabdc433092 - rustc_hir_typeck[59f56756254854d1]::used_trait_imports
  69:     0x7fabddb9431d - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::used_trait_imports, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  70:     0x7fabdd95145f - rustc_query_impl[39a2c1aa0c45d430]::get_query::used_trait_imports
  71:     0x7fabdc642979 - rustc_middle[7a2979d2db888602]::ty::query::query_get_at::<rustc_query_system[ef233bdbbc366e72]::query::caches::VecCache<rustc_hir[1ac680dbbd527d38]::hir_id::OwnerId, rustc_middle[7a2979d2db888602]::query::erase::Erased<[u8; 8usize]>>>
  72:     0x7fabdc65512c - rustc_hir_analysis[59292c9529a9ac08]::check_unused::check_crate
  73:     0x7fabdc7a0730 - rustc_hir_analysis[59292c9529a9ac08]::check_crate
  74:     0x7fabdbf69259 - rustc_interface[33255898ccb36cc3]::passes::analysis
  75:     0x7fabddc509a9 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::analysis, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  76:     0x7fabdd92c87f - rustc_query_impl[39a2c1aa0c45d430]::get_query::analysis
  77:     0x7fabdbea649d - <rustc_middle[7a2979d2db888602]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  78:     0x7fabdbed4962 - <rustc_interface[33255898ccb36cc3]::interface::Compiler>::enter::<rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}::{closure#2}, core[694345b096264287]::result::Result<core[694345b096264287]::option::Option<rustc_interface[33255898ccb36cc3]::queries::Linker>, rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  79:     0x7fabdbe9f540 - rustc_span[8be25b5ff532beb9]::set_source_map::<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  80:     0x7fabdbe987e9 - <scoped_tls[e957ced773ac1082]::ScopedKey<rustc_span[8be25b5ff532beb9]::SessionGlobals>>::set::<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
Build completed unsuccessfully in 0:13:07
  81:     0x7fabdbeaf036 - std[9214d4328e246da4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[33255898ccb36cc3]::util::run_in_thread_pool_with_globals<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  82:     0x7fabdbeeefe6 - std[9214d4328e246da4]::panicking::try::<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, core[694345b096264287]::panic::unwind_safe::AssertUnwindSafe<<std[9214d4328e246da4]::thread::Builder>::spawn_unchecked_<rustc_interface[33255898ccb36cc3]::util::run_in_thread_pool_with_globals<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  83:     0x7fabdbe9d91d - <<std[9214d4328e246da4]::thread::Builder>::spawn_unchecked_<rustc_interface[33255898ccb36cc3]::util::run_in_thread_pool_with_globals<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#1} as core[694345b096264287]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  84:     0x7fabdb3d349e - std::sys::unix::thread::Thread::new::thread_start::h1ee1e1ea471c8b55
  85:     0x7fabdb16db43 - <unknown>
  86:     0x7fabdb1ffa00 - <unknown>
  87:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (c518b25bc 2023-05-04) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `test::{constant#3}`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{constant#3}`
#2 [mir_for_ctfe] caching mir of `test::{constant#3}` for CTFE
#3 [eval_to_allocation_raw] const-evaluating + checking `test::{constant#3}`
#4 [eval_to_valtree] evaluating type-level constant
#5 [typeck] type-checking `test`
#6 [used_trait_imports] finding used_trait_imports `test`
#7 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0794`.
------------------------------------------
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-77357.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/issues/issue-77357.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Normalizing Binder(([&'a u32]; c_variadic: false)->_, []) without wrapping in a `Binder`', /checkout/compiler/rustc_trait_selection/src/traits/project.rs:437:9
   0:     0x7f6d0e4db4b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::haa8a3bcced70ba70
   1:     0x7f6d0e5428e8 - core::fmt::write::hbf4502135b21ec09
   2:     0x7f6d0e4cfb21 - std::io::Write::write_fmt::h7d9967ab0bf5a25f
   3:     0x7f6d0e4db2c1 - std::sys_common::backtrace::print::h027b8b584a5f69d4
   3:     0x7f6d0e4db2c1 - std::sys_common::backtrace::print::h027b8b584a5f69d4
   4:     0x7f6d0e4de44a - std::panicking::default_hook::{{closure}}::h9c5d941200c29b74
   5:     0x7f6d0e4de12c - std::panicking::default_hook::h83fba6cfc1469036
   6:     0x7f6d0efa1665 - rustc_driver_impl[6cef5cb93700381e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f6d0e4deb57 - std::panicking::rust_panic_with_hook::h2a1d9d43d66bf9d8
   8:     0x7f6d0e4de8d7 - std::panicking::begin_panic_handler::{{closure}}::h0fc91baf93c3414f
   9:     0x7f6d0e4db996 - std::sys_common::backtrace::__rust_end_short_backtrace::h5ec7e081352d4c42
  10:     0x7f6d0e4de5c7 - rust_begin_unwind
  11:     0x7f6d0e4930c3 - core::panicking::panic_fmt::h78ed867796ec2aec
  12:     0x7f6d0f514c80 - <rustc_trait_selection[fb5980b19f5b378b]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[7a2979d2db888602]::ty::sty::Binder<rustc_middle[7a2979d2db888602]::ty::sty::FnSig>>
  13:     0x7f6d0f51b2a9 - rustc_trait_selection[fb5980b19f5b378b]::traits::project::normalize_with_depth::<rustc_middle[7a2979d2db888602]::ty::sty::Binder<rustc_middle[7a2979d2db888602]::ty::sty::FnSig>>
  14:     0x7f6d0f648351 - <rustc_infer[3c467000369b7c4b]::infer::at::At as rustc_trait_selection[fb5980b19f5b378b]::traits::project::NormalizeExt>::normalize::<rustc_middle[7a2979d2db888602]::ty::sty::Binder<rustc_middle[7a2979d2db888602]::ty::sty::FnSig>>
  15:     0x7f6d0f488415 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::normalize::<rustc_middle[7a2979d2db888602]::ty::sty::Binder<rustc_middle[7a2979d2db888602]::ty::sty::FnSig>>
  16:     0x7f6d0f4ddff0 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::supplied_sig_of_closure
  17:     0x7f6d0f4dcb69 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::sig_of_closure_no_expectation
  18:     0x7f6d0f4d83b6 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_closure
  19:     0x7f6d0f4e1aa4 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7f6d0f479604 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7f6d0f4e0482 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7f6d0f49a7aa - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7f6d0f49ad05 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7f6d0f4e163b - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7f6d0f479604 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7f6d0f4e0482 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7f6d0f47920f - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::check_expr_coercible_to_type
  28:     0x7f6d0f54ba24 - rustc_hir_typeck[59f56756254854d1]::typeck
  29:     0x7f6d10d64106 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::typeck, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  30:     0x7f6d10a686ff - rustc_query_impl[39a2c1aa0c45d430]::get_query::typeck
  31:     0x7f6d10287c4a - rustc_mir_build[b77f9d6abbee3faf]::thir::cx::thir_body
  32:     0x7f6d10d7bc20 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::thir_body, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  33:     0x7f6d10a4abcf - rustc_query_impl[39a2c1aa0c45d430]::get_query::thir_body
  34:     0x7f6d0f42579e - rustc_ty_utils[96700c0fca5303f]::consts::thir_abstract_const
  35:     0x7f6d10cc029c - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::thir_abstract_const, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  36:     0x7f6d10a4da9e - rustc_query_impl[39a2c1aa0c45d430]::get_query::thir_abstract_const
  37:     0x7f6d11b4f2a7 - rustc_middle[7a2979d2db888602]::ty::query::query_get_at::<rustc_query_system[ef233bdbbc366e72]::query::caches::DefaultCache<rustc_span[8be25b5ff532beb9]::def_id::DefId, rustc_middle[7a2979d2db888602]::query::erase::Erased<[u8; 16usize]>>>
  38:     0x7f6d11b4c5de - <rustc_middle[7a2979d2db888602]::ty::context::TyCtxt>::bound_abstract_const
  39:     0x7f6d11c511fe - <<rustc_middle[7a2979d2db888602]::ty::context::TyCtxt>::expand_abstract_consts::Expander as rustc_type_ir[8f0c89f74918e1a9]::fold::TypeFolder<rustc_middle[7a2979d2db888602]::ty::context::TyCtxt>>::fold_const
  40:     0x7f6d116f7b56 - rustc_trait_selection[fb5980b19f5b378b]::traits::const_evaluatable::is_const_evaluatable
  41:     0x7f6d11768418 - <rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::FulfillProcessor as rustc_data_structures[362fcaf54b071d0a]::obligation_forest::ObligationProcessor>::process_obligation
  42:     0x7f6d118640d8 - <rustc_data_structures[362fcaf54b071d0a]::obligation_forest::ObligationForest<rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::FulfillProcessor>
  43:     0x7f6d117607b4 - <rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::FulfillmentContext as rustc_infer[3c467000369b7c4b]::traits::engine::TraitEngine>::select_where_possible
  44:     0x7f6d116e7722 - <dyn rustc_infer[3c467000369b7c4b]::traits::engine::TraitEngine as rustc_infer[3c467000369b7c4b]::traits::engine::TraitEngineExt>::select_all_or_error
  45:     0x7f6d1185f73d - <rustc_trait_selection[fb5980b19f5b378b]::traits::engine::ObligationCtxt>::select_all_or_error
  46:     0x7f6d0f79fd83 - rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_item_fn
  47:     0x7f6d0f796dc6 - rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_well_formed
  48:     0x7f6d10c8b147 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::check_well_formed, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  49:     0x7f6d10a8cc67 - rustc_query_impl[39a2c1aa0c45d430]::get_query::check_well_formed
  50:     0x7f6d0f86253d - std[9214d4328e246da4]::panicking::try::<(), core[694345b096264287]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[362fcaf54b071d0a]::sync::par_for_each_in<&[rustc_hir[1ac680dbbd527d38]::hir::ForeignItemId], <rustc_middle[7a2979d2db888602]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}>>
  51:     0x7f6d0f7dfb7d - rustc_data_structures[362fcaf54b071d0a]::sync::par_for_each_in::<&[rustc_hir[1ac680dbbd527d38]::hir::ItemId], <rustc_middle[7a2979d2db888602]::hir::ModuleItems>::par_items<rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  52:     0x7f6d0f7a263c - rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_mod_type_wf
  53:     0x7f6d10c89ecf - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::check_mod_type_wf, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  54:     0x7f6d10a67327 - rustc_query_impl[39a2c1aa0c45d430]::get_query::check_mod_type_wf
  55:     0x7f6d0f86266d - std[9214d4328e246da4]::panicking::try::<(), core[694345b096264287]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[362fcaf54b071d0a]::sync::par_for_each_in<&[rustc_hir[1ac680dbbd527d38]::hir_id::OwnerId], <rustc_middle[7a2979d2db888602]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[59292c9529a9ac08]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  56:     0x7f6d0f7dfced - rustc_data_structures[362fcaf54b071d0a]::sync::par_for_each_in::<&[rustc_hir[1ac680dbbd527d38]::hir_id::OwnerId], <rustc_middle[7a2979d2db888602]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[59292c9529a9ac08]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  57:     0x7f6d0f8b85f9 - rustc_hir_analysis[59292c9529a9ac08]::check_crate
  58:     0x7f6d0f081259 - rustc_interface[33255898ccb36cc3]::passes::analysis
  59:     0x7f6d10d689a9 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::analysis, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  60:     0x7f6d10a4487f - rustc_query_impl[39a2c1aa0c45d430]::get_query::analysis
  61:     0x7f6d0efbe49d - <rustc_middle[7a2979d2db888602]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  62:     0x7f6d0efec962 - <rustc_interface[33255898ccb36cc3]::interface::Compiler>::enter::<rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}::{closure#2}, core[694345b096264287]::result::Result<core[694345b096264287]::option::Option<rustc_interface[33255898ccb36cc3]::queries::Linker>, rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  63:     0x7f6d0efb7540 - rustc_span[8be25b5ff532beb9]::set_source_map::<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  64:     0x7f6d0efb07e9 - <scoped_tls[e957ced773ac1082]::ScopedKey<rustc_span[8be25b5ff532beb9]::SessionGlobals>>::set::<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  65:     0x7f6d0efc7036 - std[9214d4328e246da4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[33255898ccb36cc3]::util::run_in_thread_pool_with_globals<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  66:     0x7f6d0f006fe6 - std[9214d4328e246da4]::panicking::try::<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, core[694345b096264287]::panic::unwind_safe::AssertUnwindSafe<<std[9214d4328e246da4]::thread::Builder>::spawn_unchecked_<rustc_interface[33255898ccb36cc3]::util::run_in_thread_pool_with_globals<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  67:     0x7f6d0efb591d - <<std[9214d4328e246da4]::thread::Builder>::spawn_unchecked_<rustc_interface[33255898ccb36cc3]::util::run_in_thread_pool_with_globals<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#1} as core[694345b096264287]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  68:     0x7f6d0e4eb49e - std::sys::unix::thread::Thread::new::thread_start::h1ee1e1ea471c8b55
  69:     0x7f6d0e285b43 - <unknown>
  70:     0x7f6d0e317a00 - <unknown>
  71:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (c518b25bc 2023-05-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `bug::{constant#0}`
#1 [thir_body] building THIR for `bug::{constant#0}`
#2 [thir_abstract_const] building an abstract representation for `bug::{constant#0}`
#3 [check_well_formed] checking that `bug` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/const-generics/issues/issue-83993.rs stdout ----
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-83993.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/issues/issue-83993.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83993" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83993/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at '`WellFormed(&'b ())` has escaping bound vars, so it cannot be wrapped in a dummy binder.', compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:483:13
   0:     0x7f89ff5b94b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::haa8a3bcced70ba70
   1:     0x7f89ff6208e8 - core::fmt::write::hbf4502135b21ec09
   2:     0x7f89ff5adb21 - std::io::Write::write_fmt::h7d9967ab0bf5a25f
   3:     0x7f89ff5b92c1 - std::sys_common::backtrace::print::h027b8b584a5f69d4
   3:     0x7f89ff5b92c1 - std::sys_common::backtrace::print::h027b8b584a5f69d4
   4:     0x7f89ff5bc44a - std::panicking::default_hook::{{closure}}::h9c5d941200c29b74
   5:     0x7f89ff5bc12c - std::panicking::default_hook::h83fba6cfc1469036
   6:     0x7f8a0007f665 - rustc_driver_impl[6cef5cb93700381e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f89ff5bcb57 - std::panicking::rust_panic_with_hook::h2a1d9d43d66bf9d8
   8:     0x7f89ff5bc8d7 - std::panicking::begin_panic_handler::{{closure}}::h0fc91baf93c3414f
   9:     0x7f89ff5b9996 - std::sys_common::backtrace::__rust_end_short_backtrace::h5ec7e081352d4c42
  10:     0x7f89ff5bc5c7 - rust_begin_unwind
  11:     0x7f89ff5710c3 - core::panicking::panic_fmt::h78ed867796ec2aec
  12:     0x7f8a00648bf5 - <rustc_middle[7a2979d2db888602]::ty::sty::Binder<rustc_middle[7a2979d2db888602]::ty::PredicateKind>>::dummy
  13:     0x7f8a005679f5 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::register_wf_obligation
  14:     0x7f8a00566e30 - <rustc_hir_typeck[59f56756254854d1]::fn_ctxt::FnCtxt>::to_ty
  15:     0x7f8a0075ac80 - <rustc_hir_typeck[59f56756254854d1]::gather_locals::GatherLocalsVisitor>::declare
  16:     0x7f8a0075b2cf - <rustc_hir_typeck[59f56756254854d1]::gather_locals::GatherLocalsVisitor as rustc_hir[1ac680dbbd527d38]::intravisit::Visitor>::visit_local
  17:     0x7f8a0075a3e4 - <rustc_hir_typeck[59f56756254854d1]::gather_locals::GatherLocalsVisitor as rustc_hir[1ac680dbbd527d38]::intravisit::Visitor>::visit_block
  18:     0x7f8a00629a0e - rustc_hir_typeck[59f56756254854d1]::typeck
  19:     0x7f8a01e42106 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::typeck, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  20:     0x7f8a01b466ff - rustc_query_impl[39a2c1aa0c45d430]::get_query::typeck
  21:     0x7f8a01365c4a - rustc_mir_build[b77f9d6abbee3faf]::thir::cx::thir_body
  22:     0x7f8a01e59c20 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::thir_body, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  23:     0x7f8a01b28bcf - rustc_query_impl[39a2c1aa0c45d430]::get_query::thir_body
  24:     0x7f8a0050379e - rustc_ty_utils[96700c0fca5303f]::consts::thir_abstract_const
  25:     0x7f8a01d9e29c - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::thir_abstract_const, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  26:     0x7f8a01b2ba9e - rustc_query_impl[39a2c1aa0c45d430]::get_query::thir_abstract_const
  27:     0x7f8a02c2d2a7 - rustc_middle[7a2979d2db888602]::ty::query::query_get_at::<rustc_query_system[ef233bdbbc366e72]::query::caches::DefaultCache<rustc_span[8be25b5ff532beb9]::def_id::DefId, rustc_middle[7a2979d2db888602]::query::erase::Erased<[u8; 16usize]>>>
  28:     0x7f8a02c2a5de - <rustc_middle[7a2979d2db888602]::ty::context::TyCtxt>::bound_abstract_const
  29:     0x7f8a02d2f1fe - <<rustc_middle[7a2979d2db888602]::ty::context::TyCtxt>::expand_abstract_consts::Expander as rustc_type_ir[8f0c89f74918e1a9]::fold::TypeFolder<rustc_middle[7a2979d2db888602]::ty::context::TyCtxt>>::fold_const
  30:     0x7f8a027d5b56 - rustc_trait_selection[fb5980b19f5b378b]::traits::const_evaluatable::is_const_evaluatable
  31:     0x7f8a02846418 - <rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::FulfillProcessor as rustc_data_structures[362fcaf54b071d0a]::obligation_forest::ObligationProcessor>::process_obligation
  32:     0x7f8a029420d8 - <rustc_data_structures[362fcaf54b071d0a]::obligation_forest::ObligationForest<rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::FulfillProcessor>
  33:     0x7f8a0283e7b4 - <rustc_trait_selection[fb5980b19f5b378b]::traits::fulfill::FulfillmentContext as rustc_infer[3c467000369b7c4b]::traits::engine::TraitEngine>::select_where_possible
  34:     0x7f8a027c5722 - <dyn rustc_infer[3c467000369b7c4b]::traits::engine::TraitEngine as rustc_infer[3c467000369b7c4b]::traits::engine::TraitEngineExt>::select_all_or_error
  35:     0x7f8a0293d73d - <rustc_trait_selection[fb5980b19f5b378b]::traits::engine::ObligationCtxt>::select_all_or_error
  36:     0x7f8a0087dd83 - rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_item_fn
  37:     0x7f8a00874dc6 - rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_well_formed
  38:     0x7f8a01d69147 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::check_well_formed, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  39:     0x7f8a01b6ac67 - rustc_query_impl[39a2c1aa0c45d430]::get_query::check_well_formed
  40:     0x7f8a0094053d - std[9214d4328e246da4]::panicking::try::<(), core[694345b096264287]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[362fcaf54b071d0a]::sync::par_for_each_in<&[rustc_hir[1ac680dbbd527d38]::hir::ForeignItemId], <rustc_middle[7a2979d2db888602]::hir::ModuleItems>::par_foreign_items<rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>::{closure#0}::{closure#0}>>
  41:     0x7f8a008bdb7d - rustc_data_structures[362fcaf54b071d0a]::sync::par_for_each_in::<&[rustc_hir[1ac680dbbd527d38]::hir::ItemId], <rustc_middle[7a2979d2db888602]::hir::ModuleItems>::par_items<rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  42:     0x7f8a0088063c - rustc_hir_analysis[59292c9529a9ac08]::check::wfcheck::check_mod_type_wf
  43:     0x7f8a01d67ecf - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::check_mod_type_wf, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  44:     0x7f8a01b45327 - rustc_query_impl[39a2c1aa0c45d430]::get_query::check_mod_type_wf
  45:     0x7f8a0094066d - std[9214d4328e246da4]::panicking::try::<(), core[694345b096264287]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[362fcaf54b071d0a]::sync::par_for_each_in<&[rustc_hir[1ac680dbbd527d38]::hir_id::OwnerId], <rustc_middle[7a2979d2db888602]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[59292c9529a9ac08]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  46:     0x7f8a008bdced - rustc_data_structures[362fcaf54b071d0a]::sync::par_for_each_in::<&[rustc_hir[1ac680dbbd527d38]::hir_id::OwnerId], <rustc_middle[7a2979d2db888602]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[59292c9529a9ac08]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  47:     0x7f8a009965f9 - rustc_hir_analysis[59292c9529a9ac08]::check_crate
  48:     0x7f8a0015f259 - rustc_interface[33255898ccb36cc3]::passes::analysis
  49:     0x7f8a01e469a9 - rustc_query_system[ef233bdbbc366e72]::query::plumbing::try_execute_query::<rustc_query_impl[39a2c1aa0c45d430]::queries::analysis, rustc_query_impl[39a2c1aa0c45d430]::plumbing::QueryCtxt>
  50:     0x7f8a01b2287f - rustc_query_impl[39a2c1aa0c45d430]::get_query::analysis
  51:     0x7f8a0009c49d - <rustc_middle[7a2979d2db888602]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  52:     0x7f8a000ca962 - <rustc_interface[33255898ccb36cc3]::interface::Compiler>::enter::<rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}::{closure#2}, core[694345b096264287]::result::Result<core[694345b096264287]::option::Option<rustc_interface[33255898ccb36cc3]::queries::Linker>, rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  53:     0x7f8a00095540 - rustc_span[8be25b5ff532beb9]::set_source_map::<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  54:     0x7f8a0008e7e9 - <scoped_tls[e957ced773ac1082]::ScopedKey<rustc_span[8be25b5ff532beb9]::SessionGlobals>>::set::<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  55:     0x7f8a000a5036 - std[9214d4328e246da4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[33255898ccb36cc3]::util::run_in_thread_pool_with_globals<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>
  56:     0x7f8a000e4fe6 - std[9214d4328e246da4]::panicking::try::<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, core[694345b096264287]::panic::unwind_safe::AssertUnwindSafe<<std[9214d4328e246da4]::thread::Builder>::spawn_unchecked_<rustc_interface[33255898ccb36cc3]::util::run_in_thread_pool_with_globals<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  57:     0x7f8a0009391d - <<std[9214d4328e246da4]::thread::Builder>::spawn_unchecked_<rustc_interface[33255898ccb36cc3]::util::run_in_thread_pool_with_globals<rustc_interface[33255898ccb36cc3]::interface::run_compiler<core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>, rustc_driver_impl[6cef5cb93700381e]::run_compiler::{closure#1}>::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[694345b096264287]::result::Result<(), rustc_span[8be25b5ff532beb9]::ErrorGuaranteed>>::{closure#1} as core[694345b096264287]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7f89ff5c949e - std::sys::unix::thread::Thread::new::thread_start::h1ee1e1ea471c8b55
  59:     0x7f89ff363b43 - <unknown>
  60:     0x7f89ff3f5a00 - <unknown>
  61:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (c518b25bc 2023-05-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `bug::{constant#0}`
#1 [thir_body] building THIR for `bug::{constant#0}`
#2 [thir_abstract_const] building an abstract representation for `bug::{constant#0}`
#3 [check_well_formed] checking that `bug` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
------------------------------------------



