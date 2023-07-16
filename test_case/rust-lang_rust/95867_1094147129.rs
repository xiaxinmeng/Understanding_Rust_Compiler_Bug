plain
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
error[E0282]: type annotations needed
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.70/src/float/conv.rs:32:25
   |
32 |             x.cast() << (f_sd - exp - 1),
   |                         ^^^^^^^^^^^^^^^^ cannot infer type for type `{integer}`

thread 'rustc' panicked at 'Failed to get parent for DefId(0:0 ~ compiler_builtins[91f2])', compiler/rustc_middle/src/traits/specialization_graph.rs:45:52
stack backtrace:
   0:     0x7fe3608e27d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a88a4dc96b6b8f8
   1:     0x7fe360947468 - core::fmt::write::h42234c3e51154f4c
   2:     0x7fe3608d2ce1 - std::io::Write::write_fmt::h5505b6313bdcb0f3
   3:     0x7fe3608e5dd6 - std::panicking::default_hook::{{closure}}::hbe1bb29927e8c85b
   4:     0x7fe3608e59d5 - std::panicking::default_hook::h30c665989e20cb24
   5:     0x7fe3613f76c1 - rustc_driver[2b15ae7948b6e616]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe3608e6741 - std::panicking::rust_panic_with_hook::hab898bc6064aa4ee
   7:     0x7fe3608e6587 - std::panicking::begin_panic_handler::{{closure}}::h688434a74a3a0f4f
   8:     0x7fe3608e2d74 - std::sys_common::backtrace::__rust_end_short_backtrace::hf938ae7adc39f6fa
   9:     0x7fe3608e6279 - rust_begin_unwind
  10:     0x7fe360898dc3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7fe363c3045f - <rustc_middle[87143a0b6987fc74]::traits::specialization_graph::Ancestors>::leaf_def
  12:     0x7fe3637e5a75 - rustc_trait_selection[ee9dd18565913941]::traits::project::assoc_def
  13:     0x7fe36372cc6a - <rustc_infer[e9909202d8948eff]::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection[ee9dd18565913941]::traits::project::assemble_candidates_from_impls::{closure#0}>
  14:     0x7fe3637ee252 - rustc_trait_selection[ee9dd18565913941]::traits::project::opt_normalize_projection_type
  15:     0x7fe3637e31da - rustc_trait_selection[ee9dd18565913941]::traits::project::normalize_projection_type
  16:     0x7fe3637e133f - <rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFolder>::fold_ty
  17:     0x7fe3638a042f - <rustc_middle[87143a0b6987fc74]::ty::Ty as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFoldable>::super_fold_with::<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>
  18:     0x7fe3637e0941 - <rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFolder>::fold_ty
  19:     0x7fe361d440c4 - rustc_middle[87143a0b6987fc74]::ty::util::fold_list::<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer, rustc_middle[87143a0b6987fc74]::ty::Ty, <&rustc_middle[87143a0b6987fc74]::ty::list::List<rustc_middle[87143a0b6987fc74]::ty::Ty> as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFoldable>::try_super_fold_with<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>::{closure#0}>
  20:     0x7fe361ce88b2 - <&rustc_middle[87143a0b6987fc74]::ty::list::List<rustc_middle[87143a0b6987fc74]::ty::Ty> as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>
  21:     0x7fe361ec9531 - <rustc_middle[87143a0b6987fc74]::ty::sty::FnSig as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFoldable>::fold_with::<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>
  22:     0x7fe361eecd04 - <rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[87143a0b6987fc74]::ty::sty::FnSig>
  23:     0x7fe361ef7ba8 - rustc_trait_selection[ee9dd18565913941]::traits::project::normalize::<rustc_middle[87143a0b6987fc74]::ty::sty::FnSig>
  24:     0x7fe361dea113 - <rustc_infer[e9909202d8948eff]::infer::InferCtxt as rustc_trait_selection[ee9dd18565913941]::infer::InferCtxtExt>::partially_normalize_associated_types_in::<rustc_middle[87143a0b6987fc74]::ty::sty::FnSig>
  25:     0x7fe361c3651f - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::construct_obligation_for_trait
  26:     0x7fe361c6b106 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::lookup_method_in_trait
  27:     0x7fe361c45046 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::try_overloaded_place_op
  28:     0x7fe361c58b8d - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_kind
  29:     0x7fe361c05ad8 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  30:     0x7fe361c55649 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  31:     0x7fe361c57158 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_kind
  32:     0x7fe361c05ad8 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  33:     0x7fe361c55649 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  34:     0x7fe361c1d9ba - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_stmt
  35:     0x7fe361c1de34 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  36:     0x7fe361c56944 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_kind
  37:     0x7fe361c05ad8 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  38:     0x7fe361c55649 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  39:     0x7fe361c055cf - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_coercable_to_type
  40:     0x7fe361dc7009 - <rustc_infer[e9909202d8948eff]::infer::InferCtxtBuilder>::enter::<&rustc_middle[87143a0b6987fc74]::ty::context::TypeckResults, <rustc_typeck[435fc3ebb51e78ef]::check::inherited::InheritedBuilder>::enter<rustc_typeck[435fc3ebb51e78ef]::check::typeck_with_fallback<rustc_typeck[435fc3ebb51e78ef]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[87143a0b6987fc74]::ty::context::TypeckResults>::{closure#0}>
  41:     0x7fe361f35cae - <rustc_typeck[435fc3ebb51e78ef]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[435fc3ebb51e78ef]::check::typeck_with_fallback<rustc_typeck[435fc3ebb51e78ef]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[87143a0b6987fc74]::ty::context::TypeckResults>
  42:     0x7fe361d1ee03 - rustc_typeck[435fc3ebb51e78ef]::check::typeck
  43:     0x7fe362a58b94 - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<rustc_span[e033c2886c1ea87]::def_id::LocalDefId, &rustc_middle[87143a0b6987fc74]::ty::context::TypeckResults>>
  44:     0x7fe362b83fb0 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::typeck, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  45:     0x7fe361ebbfab - <rustc_middle[87143a0b6987fc74]::hir::map::Map>::par_body_owners::<rustc_typeck[435fc3ebb51e78ef]::check::typeck_item_bodies::{closure#0}>
  46:     0x7fe361d23a7d - rustc_typeck[435fc3ebb51e78ef]::check::typeck_item_bodies
  47:     0x7fe362a9e5df - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<(), ()>>
  48:     0x7fe362b3ad88 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::typeck_item_bodies, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  49:     0x7fe361d87710 - rustc_typeck[435fc3ebb51e78ef]::check_crate
  50:     0x7fe3614f11e1 - rustc_interface[1e8edbf255833c1]::passes::analysis
  51:     0x7fe362a935ac - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>>
  52:     0x7fe362b84af9 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::analysis, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  53:     0x7fe3613e8f6a - <rustc_interface[1e8edbf255833c1]::passes::QueryContext>::enter::<rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  54:     0x7fe36138bed0 - <rustc_interface[1e8edbf255833c1]::interface::Compiler>::enter::<rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[1e8edbf255833c1]::queries::Linker>, rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  55:     0x7fe36136f4d6 - rustc_span[e033c2886c1ea87]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#1}>
  56:     0x7fe36139eea7 - rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>
  57:     0x7fe3613a35df - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[e033c2886c1ea87]::SessionGlobals>>::set::<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  58:     0x7fe3613eaf29 - std[e4dc215d72d9f73d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  59:     0x7fe3613a5711 - std[e4dc215d72d9f73d]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  60:     0x7fe3613e5fa2 - <<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7fe3608f2df3 - std::sys::unix::thread::Thread::new::thread_start::h2f9ecc8966c8b525
  62:     0x7fe35ae43609 - start_thread
  63:     0x7fe360756163 - clone
  64:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (842581380 2022-04-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=0 -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -C panic=abort -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `int::<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.70/src/int/mod.rs:206:9: 229:10>::FUZZ_LENGTHS`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
For more information about this error, try `rustc --explain E0282`.
error: could not compile `compiler_builtins` due to previous error
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'Failed to get parent for DefId(0:0 ~ libc[f6ce])', compiler/rustc_middle/src/traits/specialization_graph.rs:45:52
   0:     0x7f7dbea5e7d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a88a4dc96b6b8f8
   1:     0x7f7dbeac3468 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f7dbea4ece1 - std::io::Write::write_fmt::h5505b6313bdcb0f3
   2:     0x7f7dbea4ece1 - std::io::Write::write_fmt::h5505b6313bdcb0f3
   3:     0x7f7dbea61dd6 - std::panicking::default_hook::{{closure}}::hbe1bb29927e8c85b
   4:     0x7f7dbea619d5 - std::panicking::default_hook::h30c665989e20cb24
   5:     0x7f7dbf5736c1 - rustc_driver[2b15ae7948b6e616]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f7dbea62741 - std::panicking::rust_panic_with_hook::hab898bc6064aa4ee
   7:     0x7f7dbea62587 - std::panicking::begin_panic_handler::{{closure}}::h688434a74a3a0f4f
   8:     0x7f7dbea5ed74 - std::sys_common::backtrace::__rust_end_short_backtrace::hf938ae7adc39f6fa
   9:     0x7f7dbea62279 - rust_begin_unwind
  10:     0x7f7dbea14dc3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7f7dc1dac45f - <rustc_middle[87143a0b6987fc74]::traits::specialization_graph::Ancestors>::leaf_def
  12:     0x7f7dc1961a75 - rustc_trait_selection[ee9dd18565913941]::traits::project::assoc_def
  13:     0x7f7dc18a8c6a - <rustc_infer[e9909202d8948eff]::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection[ee9dd18565913941]::traits::project::assemble_candidates_from_impls::{closure#0}>
  14:     0x7f7dc196a252 - rustc_trait_selection[ee9dd18565913941]::traits::project::opt_normalize_projection_type
  15:     0x7f7dc195f1da - rustc_trait_selection[ee9dd18565913941]::traits::project::normalize_projection_type
  16:     0x7f7dc195d33f - <rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFolder>::fold_ty
  17:     0x7f7dc1a1c42f - <rustc_middle[87143a0b6987fc74]::ty::Ty as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFoldable>::super_fold_with::<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>
  18:     0x7f7dc195c941 - <rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFolder>::fold_ty
  19:     0x7f7dbfec00c4 - rustc_middle[87143a0b6987fc74]::ty::util::fold_list::<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer, rustc_middle[87143a0b6987fc74]::ty::Ty, <&rustc_middle[87143a0b6987fc74]::ty::list::List<rustc_middle[87143a0b6987fc74]::ty::Ty> as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFoldable>::try_super_fold_with<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>::{closure#0}>
  20:     0x7f7dbfe648b2 - <&rustc_middle[87143a0b6987fc74]::ty::list::List<rustc_middle[87143a0b6987fc74]::ty::Ty> as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>
  21:     0x7f7dc0045531 - <rustc_middle[87143a0b6987fc74]::ty::sty::FnSig as rustc_middle[87143a0b6987fc74]::ty::fold::TypeFoldable>::fold_with::<rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>
  22:     0x7f7dc0068d04 - <rustc_trait_selection[ee9dd18565913941]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[87143a0b6987fc74]::ty::sty::FnSig>
  23:     0x7f7dc0073ba8 - rustc_trait_selection[ee9dd18565913941]::traits::project::normalize::<rustc_middle[87143a0b6987fc74]::ty::sty::FnSig>
  24:     0x7f7dbff66113 - <rustc_infer[e9909202d8948eff]::infer::InferCtxt as rustc_trait_selection[ee9dd18565913941]::infer::InferCtxtExt>::partially_normalize_associated_types_in::<rustc_middle[87143a0b6987fc74]::ty::sty::FnSig>
  25:     0x7f7dbfdb251f - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::construct_obligation_for_trait
  26:     0x7f7dbfde7106 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::lookup_method_in_trait
  27:     0x7f7dbfdc1046 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::try_overloaded_place_op
  28:     0x7f7dbfdd4b8d - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_kind
  29:     0x7f7dbfd81ad8 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  30:     0x7f7dbfdd1649 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  31:     0x7f7dbfdd5417 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_kind
  32:     0x7f7dbfd81ad8 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  33:     0x7f7dbfdd1649 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  34:     0x7f7dbfd95c0f - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_argument_types
  35:     0x7f7dbfd6ea47 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  36:     0x7f7dbfd6c947 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_call
  37:     0x7f7dbfdd2662 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_kind
  38:     0x7f7dbfd81ad8 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  39:     0x7f7dbfdd1649 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  40:     0x7f7dbfdb472f - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_overloaded_binop
  41:     0x7f7dbfdb4189 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_binop
  42:     0x7f7dbfdd30c1 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_kind
  43:     0x7f7dbfd81ad8 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  44:     0x7f7dbfdd1649 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  45:     0x7f7dbfd9948c - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_decl_initializer
  46:     0x7f7dbfd9957e - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_decl
  47:     0x7f7dbfd99723 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_stmt
  48:     0x7f7dbfd99e34 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  49:     0x7f7dbfdd2944 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_kind
  50:     0x7f7dbfd81ad8 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  51:     0x7f7dbfdd1649 - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  52:     0x7f7dbfd830cb - <rustc_typeck[435fc3ebb51e78ef]::check::fn_ctxt::FnCtxt>::check_return_expr
  53:     0x7f7dc00c2e0c - rustc_typeck[435fc3ebb51e78ef]::check::check::check_fn
  54:     0x7f7dbff42d82 - <rustc_infer[e9909202d8948eff]::infer::InferCtxtBuilder>::enter::<&rustc_middle[87143a0b6987fc74]::ty::context::TypeckResults, <rustc_typeck[435fc3ebb51e78ef]::check::inherited::InheritedBuilder>::enter<rustc_typeck[435fc3ebb51e78ef]::check::typeck_with_fallback<rustc_typeck[435fc3ebb51e78ef]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[87143a0b6987fc74]::ty::context::TypeckResults>::{closure#0}>
  55:     0x7f7dc00b1cae - <rustc_typeck[435fc3ebb51e78ef]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[435fc3ebb51e78ef]::check::typeck_with_fallback<rustc_typeck[435fc3ebb51e78ef]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[87143a0b6987fc74]::ty::context::TypeckResults>
  56:     0x7f7dbfe9ae03 - rustc_typeck[435fc3ebb51e78ef]::check::typeck
  57:     0x7f7dc0bd4b94 - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<rustc_span[e033c2886c1ea87]::def_id::LocalDefId, &rustc_middle[87143a0b6987fc74]::ty::context::TypeckResults>>
  58:     0x7f7dc0cfffb0 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::typeck, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  59:     0x7f7dc0037fab - <rustc_middle[87143a0b6987fc74]::hir::map::Map>::par_body_owners::<rustc_typeck[435fc3ebb51e78ef]::check::typeck_item_bodies::{closure#0}>
  60:     0x7f7dbfe9fa7d - rustc_typeck[435fc3ebb51e78ef]::check::typeck_item_bodies
  61:     0x7f7dc0c1a5df - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<(), ()>>
  62:     0x7f7dc0cb6d88 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::typeck_item_bodies, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  63:     0x7f7dbff03710 - rustc_typeck[435fc3ebb51e78ef]::check_crate
  64:     0x7f7dbf66d1e1 - rustc_interface[1e8edbf255833c1]::passes::analysis
  65:     0x7f7dc0c0f5ac - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>>
  66:     0x7f7dc0d00af9 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::analysis, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  67:     0x7f7dbf564f6a - <rustc_interface[1e8edbf255833c1]::passes::QueryContext>::enter::<rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  68:     0x7f7dbf507ed0 - <rustc_interface[1e8edbf255833c1]::interface::Compiler>::enter::<rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[1e8edbf255833c1]::queries::Linker>, rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  69:     0x7f7dbf4eb4d6 - rustc_span[e033c2886c1ea87]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#1}>
  70:     0x7f7dbf51aea7 - rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>
  71:     0x7f7dbf51f5df - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[e033c2886c1ea87]::SessionGlobals>>::set::<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  72:     0x7f7dbf566f29 - std[e4dc215d72d9f73d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  73:     0x7f7dbf521711 - std[e4dc215d72d9f73d]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  74:     0x7f7dbf561fa2 - <<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  75:     0x7f7dbea6edf3 - std::sys::unix::thread::Thread::new::thread_start::h2f9ecc8966c8b525
  76:     0x7f7db8fbf609 - start_thread
  77:     0x7f7dbe8d2163 - clone
  78:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (842581380 2022-04-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `unix::linux_like::FD_CLR`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/project.rs:1991:37

---
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:160:49

error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:28
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:725:18

error: internal compiler error: mir_const_qualif: MIR had errors
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_mir_transform/src/lib.rs:193:18


error: internal compiler error: PromoteTemps: MIR had errors
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:53:22


error: internal compiler error: broken MIR in DefId(0:3410 ~ libc[f6ce]::unix::linux_like::linux::ABS_CNT) ("return type"): bad type [type error]
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:540:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:795:20

error: internal compiler error: broken MIR in DefId(0:3410 ~ libc[f6ce]::unix::linux_like::linux::ABS_CNT) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1: 2993:49 (#0), scope: scope[0] } }): bad type [type error]
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:540:13

error: internal compiler error: ty::ConstKind::Error constructed but no error reported
error: internal compiler error: ty::ConstKind::Error constructed but no error reported
  |
  = note: delayed at /checkout/compiler/rustc_middle/src/ty/consts.rs:267:52

error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2091:41
     |
2091 | pub const NFNL_MSG_BATCH_END: ::c_int = NLMSG_MIN_TYPE + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2979:27
     |
2979 | pub const FF_CNT: usize = FF_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2983:35
     |
2983 | pub const INPUT_PROP_CNT: usize = INPUT_PROP_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2985:27
     |
2985 | pub const EV_CNT: usize = EV_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2987:28
     |
2987 | pub const SYN_CNT: usize = SYN_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2989:28
     |
2989 | pub const KEY_CNT: usize = KEY_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2991:28
     |
2991 | pub const REL_CNT: usize = REL_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2995:27
     |
2995 | pub const SW_CNT: usize = SW_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2997:28
     |
2997 | pub const MSC_CNT: usize = MSC_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2999:28
     |
2999 | pub const LED_CNT: usize = LED_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:3001:28
     |
3001 | pub const REP_CNT: usize = REP_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:3003:28
     |
3003 | pub const SND_CNT: usize = SND_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/gnu/mod.rs:931:40
    |
931 | pub const GENL_ID_VFS_DQUOT: ::c_int = ::NLMSG_MIN_TYPE + 1;
    |
    = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/gnu/mod.rs:932:38
    |
932 | pub const GENL_ID_PMCRAID: ::c_int = ::NLMSG_MIN_TYPE + 2;
    |
    = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

error: internal compiler error: TyKind::Error constructed but no error reported
---
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:930:53

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:902:27
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:1350:42


error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/fallback.rs:110:58

error: internal compiler error: expected fullfillment errors
  = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:210:23


error: internal compiler error: cat_expr Errd
     |
267  |  /         macro_rules! const_fn {
267  |  /         macro_rules! const_fn {
268  |  |             ($($(#[$attr:meta])* $({$constness:ident})* fn $i:ident(
269  |  |                         $($arg:ident: $argty:ty),*
270  |  |             ) -> $ret:ty {
276  |  |                 ) -> $ret {
     |  |___________________________^
     |  |___________________________^
277  | ||                     $($body);*
     | ||_________________^
279  |  |             )*)
280  |  |         }
     |  |_________- in this expansion of `const_fn!`
     |  |_________- in this expansion of `const_fn!`
     |
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1486:1
     |
1486 | /  const_fn! {
1487 | |      {const} fn CMSG_ALIGN(len: usize) -> usize {
1488 | |          len + ::mem::size_of::<usize>() - 1 & !(::mem::size_of::<usize>() - 1)
1490 | |  }
     | |__- in this macro invocation
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1488:9
     |
1488 |         len + ::mem::size_of::<usize>() - 1 & !(::mem::size_of::<usize>() - 1)
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1488:47
     |
1488 |         len + ::mem::size_of::<usize>() - 1 & !(::mem::size_of::<usize>() - 1)
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1488:48
     |
1488 |         len + ::mem::size_of::<usize>() - 1 & !(::mem::size_of::<usize>() - 1)
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1383:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1383:13
stack backtrace:
   0:     0x7f7dbea5e7d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a88a4dc96b6b8f8
   1:     0x7f7dbeac3468 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f7dbea4ece1 - std::io::Write::write_fmt::h5505b6313bdcb0f3
   3:     0x7f7dbea61dd6 - std::panicking::default_hook::{{closure}}::hbe1bb29927e8c85b
   4:     0x7f7dbea619d5 - std::panicking::default_hook::h30c665989e20cb24
   5:     0x7f7dbf5736c1 - rustc_driver[2b15ae7948b6e616]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f7dbea62741 - std::panicking::rust_panic_with_hook::hab898bc6064aa4ee
   7:     0x7f7dc1fb5de3 - std[e4dc215d72d9f73d]::panicking::begin_panic::<rustc_errors[3b46f01f9753e7d]::ExplicitBug>::{closure#0}
   8:     0x7f7dc1fb5d96 - std[e4dc215d72d9f73d]::sys_common::backtrace::__rust_end_short_backtrace::<std[e4dc215d72d9f73d]::panicking::begin_panic<rustc_errors[3b46f01f9753e7d]::ExplicitBug>::{closure#0}, !>
   9:     0x7f7dbf4c03ff - std[e4dc215d72d9f73d]::panicking::begin_panic::<rustc_errors[3b46f01f9753e7d]::ExplicitBug>
  10:     0x7f7dc1f5d186 - std[e4dc215d72d9f73d]::panic::panic_any::<rustc_errors[3b46f01f9753e7d]::ExplicitBug>
  11:     0x7f7dc1f61aea - <rustc_errors[3b46f01f9753e7d]::HandlerInner as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  12:     0x7f7dbf4fdba2 - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_session[490bd8b11d3080dc]::parse::ParseSess>
  13:     0x7f7dbf506415 - <alloc[4b492a408420e30b]::rc::Rc<rustc_session[490bd8b11d3080dc]::session::Session> as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  14:     0x7f7dbf4ed4ec - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_interface[1e8edbf255833c1]::interface::Compiler>
  15:     0x7f7dbf4ebb84 - rustc_span[e033c2886c1ea87]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f7dbf51aea7 - rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>
  17:     0x7f7dbf51f5df - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[e033c2886c1ea87]::SessionGlobals>>::set::<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  18:     0x7f7dbf566f29 - std[e4dc215d72d9f73d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  19:     0x7f7dbf521711 - std[e4dc215d72d9f73d]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f7dbf561fa2 - <<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f7dbea6edf3 - std::sys::unix::thread::Thread::new::thread_start::h2f9ecc8966c8b525
  22:     0x7f7db8fbf609 - start_thread
  23:     0x7f7dbe8d2163 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (842581380 2022-04-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)
error: could not compile `libc`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name libc /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on --cfg 'feature="align"' --cfg 'feature="rustc-dep-of-std"' --cfg 'feature="rustc-std-workspace-core"' -C metadata=73995a372eb744b2 -C extra-filename=-73995a372eb744b2 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern rustc_std_workspace_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-2deeba5fb4de62b0.rmeta --cap-lints allow -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo --cfg freebsd11 --cfg libc_priv_mod_use --cfg libc_union --cfg libc_const_size_of --cfg libc_align --cfg libc_core_cvoid --cfg libc_packedN --cfg libc_cfg_target_vendor --cfg libc_non_exhaustive --cfg libc_ptr_addr_of --cfg libc_thread_local` (exit status: 254)
