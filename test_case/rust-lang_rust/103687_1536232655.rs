
error: internal compiler error: compiler/rustc_infer/src/infer/region_constraints/mod.rs:568:17: cannot relate bound region: ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed(DefId(0:1186 ~ tui[6e6f]::callback::foo#1::'_), '_) }) <= ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed(DefId(0:1178 ~ tui[6e6f]::callback::{impl#0}::'_#1), '_) })

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/compiler/rustc_errors/src/lib.rs:987:33
stack backtrace:
   0:     0x7f2df0f6659a - std::backtrace_rs::backtrace::libunwind::trace::ha271a8a7e1f3d4ef
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f2df0f6659a - std::backtrace_rs::backtrace::trace_unsynchronized::h85739da0352c791a
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f2df0f6659a - std::sys_common::backtrace::_print_fmt::hbc6ebcfb2910b329
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f2df0f6659a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he1c117e52d53614f
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f2df0fc839e - core::fmt::write::h25eb51b9526b8e0c
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/core/src/fmt/mod.rs:1213:17
   5:     0x7f2df0f56be5 - std::io::Write::write_fmt::ha9edec5fb1621933
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/io/mod.rs:1682:15
   6:     0x7f2df0f66365 - std::sys_common::backtrace::_print::hf8657cd429fc3452
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f2df0f66365 - std::sys_common::backtrace::print::h41b9b18ed86f86bd
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f2df0f6912f - std::panicking::default_hook::{{closure}}::h22a91871f4454152
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/panicking.rs:267:22
   9:     0x7f2df0f68e6b - std::panicking::default_hook::h21ddc36de0cd4ae7
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/panicking.rs:286:9
  10:     0x7f2df42b9324 - rustc_driver[70f63b52fde826b7]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f2df0f6996a - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h6f7e3c94ecc52e2f
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/boxed.rs:2002:9
  12:     0x7f2df0f6996a - std::panicking::rust_panic_with_hook::h5059419d6d59b3d0
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/panicking.rs:692:13
  13:     0x7f2df4574f81 - std[f8b79e573431a86c]::panicking::begin_panic::<rustc_errors[53e74ce803854a4d]::ExplicitBug>::{closure#0}
  14:     0x7f2df456d8d6 - std[f8b79e573431a86c]::sys_common::backtrace::__rust_end_short_backtrace::<std[f8b79e573431a86c]::panicking::begin_panic<rustc_errors[53e74ce803854a4d]::ExplicitBug>::{closure#0}, !>
  15:     0x7f2df45fdda6 - std[f8b79e573431a86c]::panicking::begin_panic::<rustc_errors[53e74ce803854a4d]::ExplicitBug>
  16:     0x7f2df45fdd96 - std[f8b79e573431a86c]::panic::panic_any::<rustc_errors[53e74ce803854a4d]::ExplicitBug>
  17:     0x7f2df45fc6c2 - <rustc_errors[53e74ce803854a4d]::HandlerInner>::span_bug::<rustc_span[7c23fb27ec020b97]::span_encoding::Span, &alloc[b346c7f99e9347e5]::string::String>
  18:     0x7f2df45fc567 - <rustc_errors[53e74ce803854a4d]::Handler>::span_bug::<rustc_span[7c23fb27ec020b97]::span_encoding::Span, &alloc[b346c7f99e9347e5]::string::String>
  19:     0x7f2df45b1acb - rustc_middle[6dfce017f6b7786d]::util::bug::opt_span_bug_fmt::<rustc_span[7c23fb27ec020b97]::span_encoding::Span>::{closure#0}
  20:     0x7f2df45b1b1a - rustc_middle[6dfce017f6b7786d]::ty::context::tls::with_opt::<rustc_middle[6dfce017f6b7786d]::util::bug::opt_span_bug_fmt<rustc_span[7c23fb27ec020b97]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7f2df45b0bda - rustc_middle[6dfce017f6b7786d]::ty::context::tls::with_context_opt::<rustc_middle[6dfce017f6b7786d]::ty::context::tls::with_opt<rustc_middle[6dfce017f6b7786d]::util::bug::opt_span_bug_fmt<rustc_span[7c23fb27ec020b97]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7f2df45b0b06 - rustc_middle[6dfce017f6b7786d]::util::bug::opt_span_bug_fmt::<rustc_span[7c23fb27ec020b97]::span_encoding::Span>
  23:     0x7f2df45b0ac4 - rustc_middle[6dfce017f6b7786d]::util::bug::span_bug_fmt::<rustc_span[7c23fb27ec020b97]::span_encoding::Span>
  24:     0x7f2df2244604 - <rustc_middle[6dfce017f6b7786d]::ty::sty::Region as rustc_middle[6dfce017f6b7786d]::ty::relate::Relate>::relate::<rustc_infer[83c7bda8c7ffae41]::infer::equate::Equate>
  25:     0x7f2df2249ea5 - <smallvec[31f04b8acc434592]::SmallVec<[rustc_middle[6dfce017f6b7786d]::ty::subst::GenericArg; 8usize]> as core[1d432356d8e1e9f1]::iter::traits::collect::Extend<rustc_middle[6dfce017f6b7786d]::ty::subst::GenericArg>>::extend::<core[1d432356d8e1e9f1]::iter::adapters::GenericShunt<core[1d432356d8e1e9f1]::iter::adapters::map::Map<core[1d432356d8e1e9f1]::iter::adapters::zip::Zip<core[1d432356d8e1e9f1]::iter::adapters::copied::Copied<core[1d432356d8e1e9f1]::slice::iter::Iter<rustc_middle[6dfce017f6b7786d]::ty::subst::GenericArg>>, core[1d432356d8e1e9f1]::iter::adapters::copied::Copied<core[1d432356d8e1e9f1]::slice::iter::Iter<rustc_middle[6dfce017f6b7786d]::ty::subst::GenericArg>>>, rustc_middle[6dfce017f6b7786d]::ty::relate::relate_substs<rustc_infer[83c7bda8c7ffae41]::infer::equate::Equate>::{closure#0}>, core[1d432356d8e1e9f1]::result::Result<core[1d432356d8e1e9f1]::convert::Infallible, rustc_middle[6dfce017f6b7786d]::ty::error::TypeError>>>
  26:     0x7f2df223f5c8 - rustc_middle[6dfce017f6b7786d]::ty::relate::super_relate_tys::<rustc_infer[83c7bda8c7ffae41]::infer::equate::Equate>
  27:     0x7f2df223c8be - <rustc_infer[83c7bda8c7ffae41]::infer::equate::Equate as rustc_middle[6dfce017f6b7786d]::ty::relate::TypeRelation>::tys
  28:     0x7f2df2acedb3 - <rustc_infer[83c7bda8c7ffae41]::infer::InferCtxt>::commit_if_ok::<rustc_infer[83c7bda8c7ffae41]::infer::InferOk<()>, rustc_middle[6dfce017f6b7786d]::ty::error::TypeError, <rustc_infer[83c7bda8c7ffae41]::infer::at::Trace>::eq<rustc_middle[6dfce017f6b7786d]::ty::Ty>::{closure#0}>
  29:     0x7f2df2aceca6 - <rustc_infer[83c7bda8c7ffae41]::infer::at::At>::eq::<rustc_middle[6dfce017f6b7786d]::ty::Ty>
  30:     0x7f2df4e9739e - <rustc_infer[83c7bda8c7ffae41]::infer::InferCtxt>::can_eq::<rustc_middle[6dfce017f6b7786d]::ty::Ty>
  31:     0x7f2df4e7a1d2 - rustc_trait_selection[d385e741675330b5]::traits::error_reporting::suggestions::hint_missing_borrow
  32:     0x7f2df4f1d934 - <rustc_infer[83c7bda8c7ffae41]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[d385e741675330b5]::traits::error_reporting::suggestions::TypeErrCtxtExt>::report_closure_arg_mismatch
  33:     0x7f2df4f2d788 - <rustc_infer[83c7bda8c7ffae41]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[d385e741675330b5]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  34:     0x7f2df4f37167 - <rustc_infer[83c7bda8c7ffae41]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[d385e741675330b5]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  35:     0x7f2df4f2a7cf - <rustc_infer[83c7bda8c7ffae41]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[d385e741675330b5]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  36:     0x7f2df23d503d - <rustc_hir_typeck[698e9809084ff9ec]::fn_ctxt::FnCtxt>::type_inference_fallback
  37:     0x7f2df23c00cb - rustc_hir_typeck[698e9809084ff9ec]::typeck
  38:     0x7f2df23b1797 - <rustc_query_system[59807289ca97d521]::dep_graph::graph::DepGraph<rustc_middle[6dfce017f6b7786d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6dfce017f6b7786d]::ty::context::TyCtxt, rustc_span[7c23fb27ec020b97]::def_id::LocalDefId, &rustc_middle[6dfce017f6b7786d]::ty::typeck_results::TypeckResults>
  39:     0x7f2df23a92eb - rustc_query_system[59807289ca97d521]::query::plumbing::try_execute_query::<rustc_query_impl[81153385010d7886]::queries::typeck, rustc_query_impl[81153385010d7886]::plumbing::QueryCtxt>
  40:     0x7f2df3a163cd - rustc_data_structures[560608da72c35763]::sync::par_for_each_in::<&[rustc_span[7c23fb27ec020b97]::def_id::LocalDefId], <rustc_middle[6dfce017f6b7786d]::hir::map::Map>::par_body_owners<rustc_hir_typeck[698e9809084ff9ec]::typeck_item_bodies::{closure#0}>::{closure#0}>
  41:     0x7f2df3a16173 - rustc_hir_typeck[698e9809084ff9ec]::typeck_item_bodies
  42:     0x7f2df3a301d7 - <rustc_query_system[59807289ca97d521]::dep_graph::graph::DepGraph<rustc_middle[6dfce017f6b7786d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6dfce017f6b7786d]::ty::context::TyCtxt, (), ()>
  43:     0x7f2df3839c26 - rustc_query_system[59807289ca97d521]::query::plumbing::try_execute_query::<rustc_query_impl[81153385010d7886]::queries::typeck_item_bodies, rustc_query_impl[81153385010d7886]::plumbing::QueryCtxt>
  44:     0x7f2df3c71d5b - <rustc_query_impl[81153385010d7886]::Queries as rustc_middle[6dfce017f6b7786d]::ty::query::QueryEngine>::typeck_item_bodies
  45:     0x7f2df274eee2 - <rustc_session[cd7c78827427c6d6]::session::Session>::time::<(), rustc_hir_analysis[651175ae4d1746c1]::check_crate::{closure#7}>
  46:     0x7f2df274e352 - rustc_hir_analysis[651175ae4d1746c1]::check_crate
  47:     0x7f2df274dfab - rustc_interface[698bad460dbd68d3]::passes::analysis
  48:     0x7f2df39e4f76 - <rustc_query_system[59807289ca97d521]::dep_graph::graph::DepGraph<rustc_middle[6dfce017f6b7786d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6dfce017f6b7786d]::ty::context::TyCtxt, (), core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>>
  49:     0x7f2df39e4131 - rustc_query_system[59807289ca97d521]::query::plumbing::try_execute_query::<rustc_query_impl[81153385010d7886]::queries::analysis, rustc_query_impl[81153385010d7886]::plumbing::QueryCtxt>
  50:     0x7f2df3c6e60a - <rustc_query_impl[81153385010d7886]::Queries as rustc_middle[6dfce017f6b7786d]::ty::query::QueryEngine>::analysis
  51:     0x7f2df349a010 - <rustc_interface[698bad460dbd68d3]::passes::QueryContext>::enter::<rustc_driver[70f63b52fde826b7]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>>
  52:     0x7f2df3497a84 - rustc_span[7c23fb27ec020b97]::with_source_map::<core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>, rustc_interface[698bad460dbd68d3]::interface::run_compiler<core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>, rustc_driver[70f63b52fde826b7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  53:     0x7f2df34902d4 - <scoped_tls[1044df3f3db9be03]::ScopedKey<rustc_span[7c23fb27ec020b97]::SessionGlobals>>::set::<rustc_interface[698bad460dbd68d3]::interface::run_compiler<core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>, rustc_driver[70f63b52fde826b7]::run_compiler::{closure#1}>::{closure#0}, core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>>
  54:     0x7f2df348f9d2 - std[f8b79e573431a86c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[698bad460dbd68d3]::util::run_in_thread_pool_with_globals<rustc_interface[698bad460dbd68d3]::interface::run_compiler<core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>, rustc_driver[70f63b52fde826b7]::run_compiler::{closure#1}>::{closure#0}, core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>>
  55:     0x7f2df348f77a - <<std[f8b79e573431a86c]::thread::Builder>::spawn_unchecked_<rustc_interface[698bad460dbd68d3]::util::run_in_thread_pool_with_globals<rustc_interface[698bad460dbd68d3]::interface::run_compiler<core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>, rustc_driver[70f63b52fde826b7]::run_compiler::{closure#1}>::{closure#0}, core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1d432356d8e1e9f1]::result::Result<(), rustc_errors[53e74ce803854a4d]::ErrorGuaranteed>>::{closure#1} as core[1d432356d8e1e9f1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f2df0f73823 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3205ec2d7fc232c5
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/boxed.rs:1988:9
  57:     0x7f2df0f73823 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3bb5daec8177f56b
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/boxed.rs:1988:9
  58:     0x7f2df0f73823 - std::sys::unix::thread::Thread::new::thread_start::had7b8061e306bb5c
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys/unix/thread.rs:108:17
  59:     0x7f2df0c94b43 - <unknown>
  60:     0x7f2df0d26a00 - <unknown>
  61:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.2 (9eb3afe9e 2023-03-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `callback::foo`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
