
error: internal compiler error: compiler/rustc_infer/src/infer/region_constraints/mod.rs:568:17: cannot relate bound region: ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed(DefId(0:5 ~ ice[925a]::score::'_), '_) }) <= '_#11r

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/compiler/rustc_errors/src/lib.rs:987:33
stack backtrace:
   0:     0xffff79496140 - std::backtrace_rs::backtrace::libunwind::trace::hc7f75b32c0689d70
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0xffff79496140 - std::backtrace_rs::backtrace::trace_unsynchronized::h164689022fd6f73e
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0xffff79496140 - std::sys_common::backtrace::_print_fmt::he9484e79d39a3f7a
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys_common/backtrace.rs:65:5
   3:     0xffff79496140 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5e6dff1da54ef98d
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys_common/backtrace.rs:44:22
   4:     0xffff794ee000 - core::fmt::write::hc446e03b1e01cc30
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/core/src/fmt/mod.rs:1213:17
   5:     0xffff79488f50 - std::io::Write::write_fmt::h526cc884fdeb5eb2
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/io/mod.rs:1682:15
   6:     0xffff79495f4c - std::sys_common::backtrace::_print::h393dd98b7ebd466b
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys_common/backtrace.rs:47:5
   7:     0xffff79495f4c - std::sys_common::backtrace::print::hebc7712091d53c41
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys_common/backtrace.rs:34:9
   8:     0xffff79498928 - std::panicking::default_hook::{{closure}}::h39baab9c8bcc3210
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/panicking.rs:267:22
   9:     0xffff7949866c - std::panicking::default_hook::he6636537b1f4983b
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/panicking.rs:286:9
  10:     0xffff7a1ccc94 - rustc_driver[f25ea7069dadae60]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0xffff79499088 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hae11afcdab10a5eb
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/boxed.rs:2002:9
  12:     0xffff79499088 - std::panicking::rust_panic_with_hook::h2719e7603a56e8f6
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/panicking.rs:692:13
  13:     0xffff7eb2bf5c - std[43bd97b51c4b7ccc]::panicking::begin_panic::<rustc_errors[8c811b1d8d2fea79]::ExplicitBug>::{closure#0}
  14:     0xffff7eb228f4 - std[43bd97b51c4b7ccc]::sys_common::backtrace::__rust_end_short_backtrace::<std[43bd97b51c4b7ccc]::panicking::begin_panic<rustc_errors[8c811b1d8d2fea79]::ExplicitBug>::{closure#0}, !>
  15:     0xffff7a0879f0 - std[43bd97b51c4b7ccc]::panicking::begin_panic::<rustc_errors[8c811b1d8d2fea79]::ExplicitBug>
  16:     0xffff7ebf5cf8 - std[43bd97b51c4b7ccc]::panic::panic_any::<rustc_errors[8c811b1d8d2fea79]::ExplicitBug>
  17:     0xffff7ebf30e4 - <rustc_errors[8c811b1d8d2fea79]::HandlerInner>::span_bug::<rustc_span[74d55c38045fab67]::span_encoding::Span, &alloc[7ce36d2166794c95]::string::String>
  18:     0xffff7ebf2f24 - <rustc_errors[8c811b1d8d2fea79]::Handler>::span_bug::<rustc_span[74d55c38045fab67]::span_encoding::Span, &alloc[7ce36d2166794c95]::string::String>
  19:     0xffff7eb83954 - rustc_middle[ca8436e622f06ed]::util::bug::opt_span_bug_fmt::<rustc_span[74d55c38045fab67]::span_encoding::Span>::{closure#0}
  20:     0xffff7eb83c50 - rustc_middle[ca8436e622f06ed]::ty::context::tls::with_opt::<rustc_middle[ca8436e622f06ed]::util::bug::opt_span_bug_fmt<rustc_span[74d55c38045fab67]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0xffff7eb81370 - rustc_middle[ca8436e622f06ed]::ty::context::tls::with_context_opt::<rustc_middle[ca8436e622f06ed]::ty::context::tls::with_opt<rustc_middle[ca8436e622f06ed]::util::bug::opt_span_bug_fmt<rustc_span[74d55c38045fab67]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0xffff7eb81330 - rustc_middle[ca8436e622f06ed]::util::bug::opt_span_bug_fmt::<rustc_span[74d55c38045fab67]::span_encoding::Span>
  23:     0xffff7a0834c0 - rustc_middle[ca8436e622f06ed]::util::bug::span_bug_fmt::<rustc_span[74d55c38045fab67]::span_encoding::Span>
  24:     0xffff7eb79b70 - <rustc_infer[c556dcc29778f58]::infer::region_constraints::RegionConstraintCollector>::make_subregion
  25:     0xffff7eb73c30 - <rustc_infer[c556dcc29778f58]::infer::region_constraints::RegionConstraintCollector>::make_eqregion
  26:     0xffff7eb8a640 - <rustc_infer[c556dcc29778f58]::infer::equate::Equate as rustc_middle[ca8436e622f06ed]::ty::relate::TypeRelation>::regions
  27:     0xffff7eb91590 - rustc_middle[ca8436e622f06ed]::ty::relate::super_relate_tys::<rustc_infer[c556dcc29778f58]::infer::equate::Equate>
  28:     0xffff7eb8dad8 - <rustc_infer[c556dcc29778f58]::infer::equate::Equate as rustc_middle[ca8436e622f06ed]::ty::relate::TypeRelation>::tys
  29:     0xffff7eb5d7a4 - <core[3d0f08c789d39c5d]::result::Result<rustc_middle[ca8436e622f06ed]::ty::Ty, rustc_middle[ca8436e622f06ed]::ty::error::TypeError> as rustc_type_ir[e8595ee35140abc6]::InternIteratorElement<rustc_middle[ca8436e622f06ed]::ty::Ty, rustc_middle[ca8436e622f06ed]::ty::Ty>>::intern_with::<core[3d0f08c789d39c5d]::iter::adapters::map::Map<core[3d0f08c789d39c5d]::iter::adapters::zip::Zip<core[3d0f08c789d39c5d]::iter::adapters::copied::Copied<core[3d0f08c789d39c5d]::slice::iter::Iter<rustc_middle[ca8436e622f06ed]::ty::Ty>>, core[3d0f08c789d39c5d]::iter::adapters::copied::Copied<core[3d0f08c789d39c5d]::slice::iter::Iter<rustc_middle[ca8436e622f06ed]::ty::Ty>>>, rustc_middle[ca8436e622f06ed]::ty::relate::super_relate_tys<rustc_infer[c556dcc29778f58]::infer::equate::Equate>::{closure#2}>, <rustc_middle[ca8436e622f06ed]::ty::context::TyCtxt>::mk_tup<core[3d0f08c789d39c5d]::iter::adapters::map::Map<core[3d0f08c789d39c5d]::iter::adapters::zip::Zip<core[3d0f08c789d39c5d]::iter::adapters::copied::Copied<core[3d0f08c789d39c5d]::slice::iter::Iter<rustc_middle[ca8436e622f06ed]::ty::Ty>>, core[3d0f08c789d39c5d]::iter::adapters::copied::Copied<core[3d0f08c789d39c5d]::slice::iter::Iter<rustc_middle[ca8436e622f06ed]::ty::Ty>>>, rustc_middle[ca8436e622f06ed]::ty::relate::super_relate_tys<rustc_infer[c556dcc29778f58]::infer::equate::Equate>::{closure#2}>>::{closure#0}>
  30:     0xffff7eb91660 - rustc_middle[ca8436e622f06ed]::ty::relate::super_relate_tys::<rustc_infer[c556dcc29778f58]::infer::equate::Equate>
  31:     0xffff7eb8dad8 - <rustc_infer[c556dcc29778f58]::infer::equate::Equate as rustc_middle[ca8436e622f06ed]::ty::relate::TypeRelation>::tys
  32:     0xffff7e962f40 - <rustc_infer[c556dcc29778f58]::infer::InferCtxt>::commit_if_ok::<rustc_infer[c556dcc29778f58]::infer::InferOk<()>, rustc_middle[ca8436e622f06ed]::ty::error::TypeError, <rustc_infer[c556dcc29778f58]::infer::at::Trace>::eq<rustc_middle[ca8436e622f06ed]::ty::Ty>::{closure#0}>
  33:     0xffff7e96c2cc - <rustc_infer[c556dcc29778f58]::infer::InferCtxt>::can_eq::<rustc_middle[ca8436e622f06ed]::ty::Ty>
  34:     0xffff7e9454a0 - rustc_trait_selection[edb54cbc6786beb4]::traits::error_reporting::suggestions::hint_missing_borrow
  35:     0xffff7ea2fd70 - <rustc_infer[c556dcc29778f58]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[edb54cbc6786beb4]::traits::error_reporting::suggestions::TypeErrCtxtExt>::report_closure_arg_mismatch
  36:     0xffff7ea3decc - <rustc_infer[c556dcc29778f58]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[edb54cbc6786beb4]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  37:     0xffff7ea479b4 - <rustc_infer[c556dcc29778f58]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[edb54cbc6786beb4]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  38:     0xffff7ea38cdc - <rustc_infer[c556dcc29778f58]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[edb54cbc6786beb4]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  39:     0xffff7cd6fd9c - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_argument_types
  40:     0xffff7cd6f374 - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_method_argument_types
  41:     0xffff7cdaef88 - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_expr_kind
  42:     0xffff7cd630b8 - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  43:     0xffff7cd75cf0 - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_decl
  44:     0xffff7cd76134 - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_stmt
  45:     0xffff7cd76854 - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_block_with_expected
  46:     0xffff7cdaa2c8 - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_expr_kind
  47:     0xffff7cd630b8 - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  48:     0xffff7cd643b4 - <rustc_hir_typeck[f7a57d838252c150]::fn_ctxt::FnCtxt>::check_return_expr
  49:     0xffff7ce18a2c - rustc_hir_typeck[f7a57d838252c150]::check::check_fn
  50:     0xffff7ce4ec4c - rustc_hir_typeck[f7a57d838252c150]::typeck
  51:     0xffff7dff118c - rustc_query_system[ac9112f2d74ee54c]::query::plumbing::try_execute_query::<rustc_query_impl[d6b3b78047c3001e]::queries::typeck, rustc_query_impl[d6b3b78047c3001e]::plumbing::QueryCtxt>
  52:     0xffff7dd1e8a0 - <rustc_query_impl[d6b3b78047c3001e]::Queries as rustc_middle[ca8436e622f06ed]::ty::query::QueryEngine>::typeck
  53:     0xffff7ce0b0cc - rustc_data_structures[dae7e7f0f4779010]::sync::par_for_each_in::<&[rustc_span[74d55c38045fab67]::def_id::LocalDefId], <rustc_middle[ca8436e622f06ed]::hir::map::Map>::par_body_owners<rustc_hir_typeck[f7a57d838252c150]::typeck_item_bodies::{closure#0}>::{closure#0}>
  54:     0xffff7ce4cea0 - rustc_hir_typeck[f7a57d838252c150]::typeck_item_bodies
  55:     0xffff7df81fb4 - rustc_query_system[ac9112f2d74ee54c]::query::plumbing::try_execute_query::<rustc_query_impl[d6b3b78047c3001e]::queries::typeck_item_bodies, rustc_query_impl[d6b3b78047c3001e]::plumbing::QueryCtxt>
  56:     0xffff7dd1e798 - <rustc_query_impl[d6b3b78047c3001e]::Queries as rustc_middle[ca8436e622f06ed]::ty::query::QueryEngine>::typeck_item_bodies
  57:     0xffff7cfce5d0 - <rustc_session[bd5550e79ec773b]::session::Session>::time::<(), rustc_hir_analysis[a89ce9ead355c71d]::check_crate::{closure#7}>
  58:     0xffff7cf7cfb4 - rustc_hir_analysis[a89ce9ead355c71d]::check_crate
  59:     0xffff7a273e6c - rustc_interface[a23df4f4dba783b7]::passes::analysis
  60:     0xffff7dff376c - rustc_query_system[ac9112f2d74ee54c]::query::plumbing::try_execute_query::<rustc_query_impl[d6b3b78047c3001e]::queries::analysis, rustc_query_impl[d6b3b78047c3001e]::plumbing::QueryCtxt>
  61:     0xffff7dd1ba50 - <rustc_query_impl[d6b3b78047c3001e]::Queries as rustc_middle[ca8436e622f06ed]::ty::query::QueryEngine>::analysis
  62:     0xffff7a162bd0 - <rustc_interface[a23df4f4dba783b7]::passes::QueryContext>::enter::<rustc_driver[f25ea7069dadae60]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>>
  63:     0xffff7a145414 - <rustc_interface[a23df4f4dba783b7]::queries::QueryResult<rustc_interface[a23df4f4dba783b7]::passes::QueryContext>>::enter::<core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>, rustc_driver[f25ea7069dadae60]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
  64:     0xffff7a1a7a88 - rustc_span[74d55c38045fab67]::with_source_map::<core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>, rustc_interface[a23df4f4dba783b7]::interface::run_compiler<core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>, rustc_driver[f25ea7069dadae60]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  65:     0xffff7a168b58 - std[43bd97b51c4b7ccc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a23df4f4dba783b7]::util::run_in_thread_pool_with_globals<rustc_interface[a23df4f4dba783b7]::interface::run_compiler<core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>, rustc_driver[f25ea7069dadae60]::run_compiler::{closure#1}>::{closure#0}, core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>>
  66:     0xffff7a149930 - <<std[43bd97b51c4b7ccc]::thread::Builder>::spawn_unchecked_<rustc_interface[a23df4f4dba783b7]::util::run_in_thread_pool_with_globals<rustc_interface[a23df4f4dba783b7]::interface::run_compiler<core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>, rustc_driver[f25ea7069dadae60]::run_compiler::{closure#1}>::{closure#0}, core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3d0f08c789d39c5d]::result::Result<(), rustc_errors[8c811b1d8d2fea79]::ErrorGuaranteed>>::{closure#1} as core[3d0f08c789d39c5d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  67:     0xffff794a2234 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h58accc842c8ef0c5
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/boxed.rs:1988:9
  68:     0xffff794a2234 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9aa66d4569106562
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/boxed.rs:1988:9
  69:     0xffff794a2234 - std::sys::unix::thread::Thread::new::thread_start::h7845f197aceb6559
                               at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys/unix/thread.rs:108:17
  70:     0xffff792bd5c8 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  71:     0xffff79325d1c - thread_start
                               at ./misc/../sysdeps/unix/sysv/linux/aarch64/clone.S:79
  72:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.2 (9eb3afe9e 2023-03-27) running on aarch64-unknown-linux-gnu

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
