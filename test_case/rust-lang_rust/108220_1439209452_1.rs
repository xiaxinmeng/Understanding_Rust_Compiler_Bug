
$ cargo +stable build
   Compiling mcve v0.1.0 (/tmp/mcve)
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> src/lib.rs:1:1
  |
1 | #![feature(associated_const_equality)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'assertion failed: matches!(self.def_kind(ct.def.did), DefKind :: AnonConst)', compiler/rustc_middle/src/mir/interpret/queries.rs:97:25
stack backtrace:
   0:     0x7f0bb93886fa - std::backtrace_rs::backtrace::libunwind::trace::h79937bc171ada62c
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f0bb93886fa - std::backtrace_rs::backtrace::trace_unsynchronized::h2292bca8571cb919
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f0bb93886fa - std::sys_common::backtrace::_print_fmt::h9c461f248e4ae90d
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f0bb93886fa - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9fe6bf1a39182e1
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f0bb93eb25e - core::fmt::write::h032658c119c720d7
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f0bb9378a85 - std::io::Write::write_fmt::h299fc90dfae41c0d
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/io/mod.rs:1682:15
   6:     0x7f0bb93884c5 - std::sys_common::backtrace::_print::heb70d25df9937e3f
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f0bb93884c5 - std::sys_common::backtrace::print::had745c0a76b8b521
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f0bb938b20f - std::panicking::default_hook::{{closure}}::h1ea782cdfa2fd097
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:267:22
   9:     0x7f0bb938af4b - std::panicking::default_hook::h1cc3af63455a163c
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:286:9
  10:     0x7f0bbc683ab1 - <rustc_driver[5c3b90d1fb3964ba]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[e6a29f2585b3d454]::ops::function::FnOnce<(&core[e6a29f2585b3d454]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f0bb938ba4d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h6e4950ba7c0fd82a
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/alloc/src/boxed.rs:2032:9
  12:     0x7f0bb938ba4d - std::panicking::rust_panic_with_hook::h5cafdc4b3bfd5528
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:692:13
  13:     0x7f0bb938b782 - std::panicking::begin_panic_handler::{{closure}}::hf31c60f40775892c
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:577:13
  14:     0x7f0bb9388bac - std::sys_common::backtrace::__rust_end_short_backtrace::h28a5c7be595826cd
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7f0bb938b4d2 - rust_begin_unwind
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:575:5
  16:     0x7f0bb93e7c43 - core::panicking::panic_fmt::h8fa27a0b37dd98b7
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/panicking.rs:64:14
  17:     0x7f0bb93e7d1d - core::panicking::panic::h545818946343732b
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/panicking.rs:111:5
  18:     0x7f0bbad5f549 - <rustc_middle[83f907612b22699d]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  19:     0x7f0bbad5eb51 - <rustc_middle[83f907612b22699d]::ty::consts::Const>::eval
  20:     0x7f0bbad5ac02 - <rustc_trait_selection[945b70100f8dd3ec]::traits::project::AssocTypeNormalizer as rustc_middle[83f907612b22699d]::ty::fold::TypeFolder>::fold_const
  21:     0x7f0bbad4c81d - rustc_trait_selection[945b70100f8dd3ec]::traits::project::opt_normalize_projection_type
  22:     0x7f0bbae29b48 - <rustc_infer[ce51c2dc4d9f2303]::infer::InferCtxt>::commit_if_ok::<rustc_trait_selection[945b70100f8dd3ec]::traits::project::ProjectAndUnifyResult, rustc_infer[ce51c2dc4d9f2303]::traits::project::MismatchedProjectionTypes, rustc_trait_selection[945b70100f8dd3ec]::traits::project::poly_project_and_unify_type::{closure#0}>
  23:     0x7f0bbaf6104d - <rustc_trait_selection[945b70100f8dd3ec]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[89959c1738bdde42]::vec::into_iter::IntoIter<rustc_infer[ce51c2dc4d9f2303]::traits::Obligation<rustc_middle[83f907612b22699d]::ty::Predicate>>>
  24:     0x7f0bbaf5ed51 - <rustc_infer[ce51c2dc4d9f2303]::infer::InferCtxt>::probe::<core[e6a29f2585b3d454]::result::Result<rustc_middle[83f907612b22699d]::traits::select::EvaluationResult, rustc_middle[83f907612b22699d]::traits::select::OverflowError>, <rustc_trait_selection[945b70100f8dd3ec]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[945b70100f8dd3ec]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  25:     0x7f0bbaf7c47e - <alloc[89959c1738bdde42]::vec::Vec<rustc_trait_selection[945b70100f8dd3ec]::traits::select::EvaluatedCandidate> as alloc[89959c1738bdde42]::vec::spec_from_iter::SpecFromIter<rustc_trait_selection[945b70100f8dd3ec]::traits::select::EvaluatedCandidate, core[e6a29f2585b3d454]::iter::adapters::GenericShunt<core[e6a29f2585b3d454]::iter::adapters::flatten::FlatMap<core[e6a29f2585b3d454]::iter::adapters::map::Map<alloc[89959c1738bdde42]::vec::into_iter::IntoIter<rustc_middle[83f907612b22699d]::traits::select::SelectionCandidate>, <rustc_trait_selection[945b70100f8dd3ec]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[e6a29f2585b3d454]::option::Option<core[e6a29f2585b3d454]::result::Result<rustc_trait_selection[945b70100f8dd3ec]::traits::select::EvaluatedCandidate, rustc_middle[83f907612b22699d]::traits::SelectionError>>, <core[e6a29f2585b3d454]::result::Result<core[e6a29f2585b3d454]::option::Option<rustc_trait_selection[945b70100f8dd3ec]::traits::select::EvaluatedCandidate>, rustc_middle[83f907612b22699d]::traits::SelectionError>>::transpose>, core[e6a29f2585b3d454]::result::Result<core[e6a29f2585b3d454]::convert::Infallible, rustc_middle[83f907612b22699d]::traits::SelectionError>>>>::from_iter
  26:     0x7f0bbaf79850 - <rustc_trait_selection[945b70100f8dd3ec]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  27:     0x7f0bba8ddbdb - <rustc_trait_selection[945b70100f8dd3ec]::traits::select::SelectionContext>::select
  28:     0x7f0bba8cd091 - <rustc_trait_selection[945b70100f8dd3ec]::traits::fulfill::FulfillProcessor as rustc_data_structures[1026114362f98086]::obligation_forest::ObligationProcessor>::process_obligation
  29:     0x7f0bba8ca7a2 - <rustc_data_structures[1026114362f98086]::obligation_forest::ObligationForest<rustc_trait_selection[945b70100f8dd3ec]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[945b70100f8dd3ec]::traits::fulfill::FulfillProcessor>
  30:     0x7f0bbb16aafb - <rustc_trait_selection[945b70100f8dd3ec]::traits::engine::ObligationCtxt>::select_all_or_error
  31:     0x7f0bbb6a50dd - rustc_hir_analysis[44326659f1d3f01d]::check::wfcheck::check_well_formed
  32:     0x7f0bba88328c - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[83f907612b22699d]::ty::context::TyCtxt, rustc_hir[dd085327af2d6b95]::hir_id::OwnerId, ()>
  33:     0x7f0bba881ce0 - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::get_query::<rustc_query_impl[e214cefb6de2a99d]::queries::check_well_formed, rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  34:     0x7f0bbba55608 - rustc_data_structures[1026114362f98086]::sync::par_for_each_in::<&[rustc_hir[dd085327af2d6b95]::hir::ImplItemId], <rustc_middle[83f907612b22699d]::hir::ModuleItems>::par_impl_items<rustc_hir_analysis[44326659f1d3f01d]::check::wfcheck::check_mod_type_wf::{closure#1}>::{closure#0}>
  35:     0x7f0bbba55417 - rustc_hir_analysis[44326659f1d3f01d]::check::wfcheck::check_mod_type_wf
  36:     0x7f0bbb22b77c - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[83f907612b22699d]::ty::context::TyCtxt, rustc_span[41a321a6411ba4fa]::def_id::LocalDefId, ()>
  37:     0x7f0bbb22968e - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::try_execute_query::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt, rustc_query_system[7dbbccfee5a2d054]::query::caches::VecCache<rustc_span[41a321a6411ba4fa]::def_id::LocalDefId, ()>>
  38:     0x7f0bbba25ce3 - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::get_query::<rustc_query_impl[e214cefb6de2a99d]::queries::check_mod_type_wf, rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  39:     0x7f0bbab7e5e8 - rustc_data_structures[1026114362f98086]::sync::par_for_each_in::<&[rustc_hir[dd085327af2d6b95]::hir_id::OwnerId], <rustc_middle[83f907612b22699d]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[44326659f1d3f01d]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  40:     0x7f0bbab7e3d3 - <rustc_session[b89b9f24749004e7]::session::Session>::track_errors::<rustc_hir_analysis[44326659f1d3f01d]::check_crate::{closure#5}, ()>
  41:     0x7f0bbab7dced - rustc_hir_analysis[44326659f1d3f01d]::check_crate
  42:     0x7f0bbab7d98b - rustc_interface[65dcc5dffb099e04]::passes::analysis
  43:     0x7f0bbbe3591f - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[83f907612b22699d]::ty::context::TyCtxt, (), core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  44:     0x7f0bbbe34a17 - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::try_execute_query::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt, rustc_query_system[7dbbccfee5a2d054]::query::caches::DefaultCache<(), core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>>
  45:     0x7f0bbbe34470 - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::get_query::<rustc_query_impl[e214cefb6de2a99d]::queries::analysis, rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  46:     0x7f0bbb8471b3 - <rustc_interface[65dcc5dffb099e04]::passes::QueryContext>::enter::<rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  47:     0x7f0bbb843733 - <rustc_interface[65dcc5dffb099e04]::interface::Compiler>::enter::<rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}::{closure#2}, core[e6a29f2585b3d454]::result::Result<core[e6a29f2585b3d454]::option::Option<rustc_interface[65dcc5dffb099e04]::queries::Linker>, rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  48:     0x7f0bbb83e788 - rustc_span[41a321a6411ba4fa]::with_source_map::<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_interface[65dcc5dffb099e04]::interface::run_compiler<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  49:     0x7f0bbb83e275 - <scoped_tls[393dd8f8fd825c8d]::ScopedKey<rustc_span[41a321a6411ba4fa]::SessionGlobals>>::set::<rustc_interface[65dcc5dffb099e04]::interface::run_compiler<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}>::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  50:     0x7f0bbb83d862 - std[359ab902947f5f0b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[65dcc5dffb099e04]::util::run_in_thread_pool_with_globals<rustc_interface[65dcc5dffb099e04]::interface::run_compiler<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}>::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  51:     0x7f0bbbf36b7a - <<std[359ab902947f5f0b]::thread::Builder>::spawn_unchecked_<rustc_interface[65dcc5dffb099e04]::util::run_in_thread_pool_with_globals<rustc_interface[65dcc5dffb099e04]::interface::run_compiler<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}>::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>::{closure#1} as core[e6a29f2585b3d454]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7f0bb9395803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb77d8d72ebcf79c4
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/alloc/src/boxed.rs:2000:9
  53:     0x7f0bb9395803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc08c3353e1568487
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/alloc/src/boxed.rs:2000:9
  54:     0x7f0bb9395803 - std::sys::unix::thread::Thread::new::thread_start::h7168e596cd5e5ce6
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys/unix/thread.rs:108:17
  55:     0x7f0bb90ffbb5 - <unknown>
  56:     0x7f0bb9181d90 - <unknown>
  57:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.1 (d5a82bbd2 2023-02-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED] -C target-cpu=native

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_well_formed] checking that `<impl at src/lib.rs:29:1: 29:26>` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0554`.
error: could not compile `mcve` due to previous error
$
