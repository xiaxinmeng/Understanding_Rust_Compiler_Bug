plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:768:9: const parameter `COOP_PREFERRED/#2` (Const { ty: bool, kind: Param(COOP_PREFERRED/#2) }/2) out of range when substituting substs=[_, alloc::Global]

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/compiler/rustc_errors/src/lib.rs:1519:9
stack backtrace:
   0:     0x7f82c9952340 - std::backtrace_rs::backtrace::libunwind::trace::h2df8645d586d9354
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f82c9952340 - std::backtrace_rs::backtrace::trace_unsynchronized::h8ccc7432adfc32a8
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f82c9952340 - std::sys_common::backtrace::_print_fmt::h6b5315b3e6c5b112
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f82c9952340 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1196a40d5ac35d56
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f82c5b0f98e - core::fmt::write::hb5395aee3db34e90
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f82c99460f5 - std::io::Write::write_fmt::hf1b72fac8a3acb27
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/io/mod.rs:1682:15
   6:     0x7f82c9952105 - std::sys_common::backtrace::_print::h805a3d8a840d4dd9
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f82c9952105 - std::sys_common::backtrace::print::h9dc5789e99bcd646
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f82c995445f - std::panicking::default_hook::{{closure}}::h79c2a100d70e0a69
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:267:22
   9:     0x7f82c995419a - std::panicking::default_hook::h9d54a6e2e183091b
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:286:9
  10:     0x7f82c8c1a164 - rustc_driver[2731657a57c33a9f]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f82c9954c49 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h22fa2a084fbafa5c
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/alloc/src/boxed.rs:2001:9
  12:     0x7f82c9954c49 - std::panicking::rust_panic_with_hook::h66c09c3756b56830
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:692:13
  13:     0x7f82c9042ef1 - std[6a238fb43aa4c274]::panicking::begin_panic::<rustc_errors[8289f13c10dc8783]::ExplicitBug>::{closure#0}
  14:     0x7f82c9042a56 - std[6a238fb43aa4c274]::sys_common::backtrace::__rust_end_short_backtrace::<std[6a238fb43aa4c274]::panicking::begin_panic<rustc_errors[8289f13c10dc8783]::ExplicitBug>::{closure#0}, !>
  15:     0x7f82c90a80a6 - std[6a238fb43aa4c274]::panicking::begin_panic::<rustc_errors[8289f13c10dc8783]::ExplicitBug>
  16:     0x7f82c9041a26 - std[6a238fb43aa4c274]::panic::panic_any::<rustc_errors[8289f13c10dc8783]::ExplicitBug>
  17:     0x7f82c904193d - <rustc_errors[8289f13c10dc8783]::HandlerInner>::bug::<&alloc[5aec1d928cb65a40]::string::String>
  18:     0x7f82c90413b0 - <rustc_errors[8289f13c10dc8783]::Handler>::bug::<&alloc[5aec1d928cb65a40]::string::String>
  19:     0x7f82c910bbfd - rustc_middle[1e440ea759c2f365]::ty::context::tls::with_context_opt::<rustc_middle[1e440ea759c2f365]::ty::context::tls::with_opt<rustc_middle[1e440ea759c2f365]::util::bug::opt_span_bug_fmt<rustc_span[85bea1f81190796e]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7f82c910ddb6 - rustc_middle[1e440ea759c2f365]::util::bug::opt_span_bug_fmt::<rustc_span[85bea1f81190796e]::span_encoding::Span>
  21:     0x7f82c6e9c4e3 - rustc_middle[1e440ea759c2f365]::util::bug::bug_fmt
  22:     0x7f82c90f48e2 - <rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder>::const_param_out_of_range
  23:     0x7f82c6c79d8d - <rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder as rustc_middle[1e440ea759c2f365]::ty::fold::TypeFolder>::fold_const
  24:     0x7f82c6c7b5dc - <&rustc_middle[1e440ea759c2f365]::ty::list::List<rustc_middle[1e440ea759c2f365]::ty::subst::GenericArg> as rustc_middle[1e440ea759c2f365]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder>
  25:     0x7f82c6c7bf1c - <rustc_middle[1e440ea759c2f365]::ty::consts::Const as rustc_middle[1e440ea759c2f365]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder>
  26:     0x7f82c6c77966 - <rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder as rustc_middle[1e440ea759c2f365]::ty::fold::FallibleTypeFolder>::try_fold_ty
  27:     0x7f82c90f316f - <rustc_middle[1e440ea759c2f365]::ty::subst::GenericArg as rustc_middle[1e440ea759c2f365]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder>
  28:     0x7f82c6d0a96c - <rustc_middle[1e440ea759c2f365]::ty::generics::GenericPredicates>::instantiate_into
  29:     0x7f82c6e755fc - <rustc_trait_selection[429aeeaf97ad5f33]::traits::wf::WfPredicates>::nominal_obligations_inner
  30:     0x7f82c6e71e9b - <rustc_trait_selection[429aeeaf97ad5f33]::traits::wf::WfPredicates>::compute
  31:     0x7f82c7b2a4b3 - rustc_trait_selection[429aeeaf97ad5f33]::traits::wf::obligations
  32:     0x7f82c7677b35 - <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[5aec1d928cb65a40]::vec::into_iter::IntoIter<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>>
  33:     0x7f82c7672996 - <rustc_infer[eb3d8a9fc270db4a]::infer::InferCtxt>::probe::<core[6f75f940baff7b16]::result::Result<rustc_middle[1e440ea759c2f365]::traits::select::EvaluationResult, rustc_middle[1e440ea759c2f365]::traits::select::OverflowError>, <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  34:     0x7f82c766e6e2 - <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  35:     0x7f82c6e78094 - <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::select
  36:     0x7f82c7424789 - rustc_trait_selection[429aeeaf97ad5f33]::traits::project::opt_normalize_projection_type
  37:     0x7f82c73db824 - <rustc_infer[eb3d8a9fc270db4a]::infer::InferCtxt>::commit_if_ok::<rustc_trait_selection[429aeeaf97ad5f33]::traits::project::ProjectAndUnifyResult, rustc_infer[eb3d8a9fc270db4a]::traits::project::MismatchedProjectionTypes, rustc_trait_selection[429aeeaf97ad5f33]::traits::project::poly_project_and_unify_type::{closure#0}>
  38:     0x7f82c773b85d - <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluate_predicate_recursively
  39:     0x7f82c814772c - <rustc_infer[eb3d8a9fc270db4a]::infer::InferCtxt>::probe::<core[6f75f940baff7b16]::result::Result<rustc_middle[1e440ea759c2f365]::traits::select::EvaluationResult, rustc_middle[1e440ea759c2f365]::traits::select::OverflowError>, <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  40:     0x7f82c815f769 - <core[6f75f940baff7b16]::iter::adapters::copied::Copied<core[6f75f940baff7b16]::slice::iter::Iter<rustc_middle[1e440ea759c2f365]::ty::Predicate>> as core[6f75f940baff7b16]::iter::traits::iterator::Iterator>::try_fold::<(), &mut core[6f75f940baff7b16]::iter::adapters::map::map_try_fold<rustc_middle[1e440ea759c2f365]::ty::Predicate, rustc_middle[1e440ea759c2f365]::ty::Predicate, (), core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#0}, core[6f75f940baff7b16]::iter::adapters::map::map_try_fold<rustc_middle[1e440ea759c2f365]::ty::Predicate, rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>, (), core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#1}, core[6f75f940baff7b16]::iter::traits::iterator::Iterator::find::check<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>, &mut rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#2}>::{closure#0}>::{closure#0}>::{closure#0}, core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>>
  41:     0x7f82c815f503 - <core[6f75f940baff7b16]::iter::adapters::chain::Chain<core[6f75f940baff7b16]::iter::adapters::copied::Copied<core[6f75f940baff7b16]::slice::iter::Iter<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, alloc[5aec1d928cb65a40]::vec::into_iter::IntoIter<rustc_middle[1e440ea759c2f365]::ty::Predicate>> as core[6f75f940baff7b16]::iter::traits::iterator::Iterator>::try_fold::<(), core[6f75f940baff7b16]::iter::adapters::map::map_try_fold<rustc_middle[1e440ea759c2f365]::ty::Predicate, rustc_middle[1e440ea759c2f365]::ty::Predicate, (), core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#0}, core[6f75f940baff7b16]::iter::adapters::map::map_try_fold<rustc_middle[1e440ea759c2f365]::ty::Predicate, rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>, (), core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#1}, core[6f75f940baff7b16]::iter::traits::iterator::Iterator::find::check<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>, &mut rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#2}>::{closure#0}>::{closure#0}>::{closure#0}, core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>>
  42:     0x7f82c7fa3467 - rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::overlap_within_probe
  43:     0x7f82c7fa28d3 - <rustc_infer[eb3d8a9fc270db4a]::infer::InferCtxt>::probe_maybe_skip_leak_check::<core[6f75f940baff7b16]::option::Option<rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::OverlapResult>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::overlap::{closure#0}>
  44:     0x7f82c7f9fb8c - <rustc_middle[1e440ea759c2f365]::traits::specialization_graph::Children as rustc_trait_selection[429aeeaf97ad5f33]::traits::specialize::specialization_graph::ChildrenExt>::insert
  45:     0x7f82c7f9ccd3 - <rustc_middle[1e440ea759c2f365]::traits::specialization_graph::Graph as rustc_trait_selection[429aeeaf97ad5f33]::traits::specialize::specialization_graph::GraphExt>::insert
  46:     0x7f82c7f9ae3d - rustc_trait_selection[429aeeaf97ad5f33]::traits::specialize::specialization_graph_provider
  47:     0x7f82c7f573f8 - rustc_query_system[9e10ef674197eb88]::query::plumbing::try_execute_query::<rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt, rustc_query_system[9e10ef674197eb88]::query::caches::ArenaCache<rustc_span[85bea1f81190796e]::def_id::DefId, rustc_middle[1e440ea759c2f365]::traits::specialization_graph::Graph>>
  48:     0x7f82c7ec0e5e - rustc_query_system[9e10ef674197eb88]::query::plumbing::get_query::<rustc_query_impl[94dddaa624b13a2e]::queries::specialization_graph_of, rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt>
  49:     0x7f82c829393b - rustc_hir_analysis[9182be28d676711c]::coherence::coherent_trait
  50:     0x7f82c75d9dc8 - rustc_query_system[9e10ef674197eb88]::query::plumbing::try_execute_query::<rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt, rustc_query_system[9e10ef674197eb88]::query::caches::DefaultCache<rustc_span[85bea1f81190796e]::def_id::DefId, ()>>
  51:     0x7f82c7f84c65 - rustc_query_system[9e10ef674197eb88]::query::plumbing::get_query::<rustc_query_impl[94dddaa624b13a2e]::queries::coherent_trait, rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt>
  52:     0x7f82c704afa2 - <rustc_session[638750f383577479]::session::Session>::track_errors::<rustc_hir_analysis[9182be28d676711c]::check_crate::{closure#3}, ()>
  53:     0x7f82c7047003 - rustc_hir_analysis[9182be28d676711c]::check_crate
  54:     0x7f82c7046cdb - rustc_interface[e10a04643064e0c3]::passes::analysis
  55:     0x7f82c8482cb4 - rustc_query_system[9e10ef674197eb88]::query::plumbing::try_execute_query::<rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt, rustc_query_system[9e10ef674197eb88]::query::caches::DefaultCache<(), core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>>
  56:     0x7f82c84829b7 - rustc_query_system[9e10ef674197eb88]::query::plumbing::get_query::<rustc_query_impl[94dddaa624b13a2e]::queries::analysis, rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt>
  57:     0x7f82c7fc56fd - <rustc_interface[e10a04643064e0c3]::passes::QueryContext>::enter::<rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  58:     0x7f82c7fc1baf - <rustc_interface[e10a04643064e0c3]::interface::Compiler>::enter::<rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}::{closure#2}, core[6f75f940baff7b16]::result::Result<core[6f75f940baff7b16]::option::Option<rustc_interface[e10a04643064e0c3]::queries::Linker>, rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  59:     0x7f82c7fb96f2 - rustc_span[85bea1f81190796e]::with_source_map::<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  60:     0x7f82c7fb91e9 - <scoped_tls[df6a59750083ca9d]::ScopedKey<rustc_span[85bea1f81190796e]::SessionGlobals>>::set::<rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  61:     0x7f82c7fb87f8 - std[6a238fb43aa4c274]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e10a04643064e0c3]::util::run_in_thread_pool_with_globals<rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  62:     0x7f82c7fb851c - <<std[6a238fb43aa4c274]::thread::Builder>::spawn_unchecked_<rustc_interface[e10a04643064e0c3]::util::run_in_thread_pool_with_globals<rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>::{closure#1} as core[6f75f940baff7b16]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  63:     0x7f82c995bbc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h23aa6e7db304ed51
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/alloc/src/boxed.rs:1987:9
  64:     0x7f82c995bbc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0482f1835c06f38d
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/alloc/src/boxed.rs:1987:9
  65:     0x7f82c995bbc3 - std::sys::unix::thread::Thread::new::thread_start::h5213f0bce91e8f3e
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys/unix/thread.rs:108:17
  66:     0x7f82c5817b43 - <unknown>
  67:     0x7f82c58a9a00 - <unknown>
  68:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-beta.1 (e080cc5a6 2022-11-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [specialization_graph_of] building specialization graph of trait `collections::SpecExtend`
#1 [coherent_trait] coherence checking all impls of trait `collections::SpecExtend`
#2 [analysis] running analysis passes on this crate
error: could not compile `alloc`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:16
