plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v1.0.0
    Checking adler v1.0.2
    Checking rustc-demangle v0.1.21
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:768:9: const parameter `COOP_PREFERRED/#2` (Const { ty: bool, kind: Param(COOP_PREFERRED/#2) }/2) out of range when substituting substs=[_, alloc::Global]

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/compiler/rustc_errors/src/lib.rs:1519:9
stack backtrace:
   0:     0x7f28c073e340 - std::backtrace_rs::backtrace::libunwind::trace::h2df8645d586d9354
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f28c073e340 - std::backtrace_rs::backtrace::trace_unsynchronized::h8ccc7432adfc32a8
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f28c073e340 - std::sys_common::backtrace::_print_fmt::h6b5315b3e6c5b112
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f28c073e340 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1196a40d5ac35d56
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f28c0c2898e - core::fmt::write::hb5395aee3db34e90
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f28c07320f5 - std::io::Write::write_fmt::hf1b72fac8a3acb27
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/io/mod.rs:1682:15
   6:     0x7f28c073e105 - std::sys_common::backtrace::_print::h805a3d8a840d4dd9
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f28c073e105 - std::sys_common::backtrace::print::h9dc5789e99bcd646
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f28c074045f - std::panicking::default_hook::{{closure}}::h79c2a100d70e0a69
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:267:22
   9:     0x7f28c074019a - std::panicking::default_hook::h9d54a6e2e183091b
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:286:9
  10:     0x7f28bfa06164 - rustc_driver[2731657a57c33a9f]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f28c0740c49 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h22fa2a084fbafa5c
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/alloc/src/boxed.rs:2001:9
  12:     0x7f28c0740c49 - std::panicking::rust_panic_with_hook::h66c09c3756b56830
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/panicking.rs:692:13
  13:     0x7f28bfe2eef1 - std[6a238fb43aa4c274]::panicking::begin_panic::<rustc_errors[8289f13c10dc8783]::ExplicitBug>::{closure#0}
  14:     0x7f28bfe2ea56 - std[6a238fb43aa4c274]::sys_common::backtrace::__rust_end_short_backtrace::<std[6a238fb43aa4c274]::panicking::begin_panic<rustc_errors[8289f13c10dc8783]::ExplicitBug>::{closure#0}, !>
  15:     0x7f28bfe940a6 - std[6a238fb43aa4c274]::panicking::begin_panic::<rustc_errors[8289f13c10dc8783]::ExplicitBug>
  16:     0x7f28bfe2da26 - std[6a238fb43aa4c274]::panic::panic_any::<rustc_errors[8289f13c10dc8783]::ExplicitBug>
  17:     0x7f28bfe2d93d - <rustc_errors[8289f13c10dc8783]::HandlerInner>::bug::<&alloc[5aec1d928cb65a40]::string::String>
  18:     0x7f28bfe2d3b0 - <rustc_errors[8289f13c10dc8783]::Handler>::bug::<&alloc[5aec1d928cb65a40]::string::String>
  19:     0x7f28bfef7bfd - rustc_middle[1e440ea759c2f365]::ty::context::tls::with_context_opt::<rustc_middle[1e440ea759c2f365]::ty::context::tls::with_opt<rustc_middle[1e440ea759c2f365]::util::bug::opt_span_bug_fmt<rustc_span[85bea1f81190796e]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7f28bfef9db6 - rustc_middle[1e440ea759c2f365]::util::bug::opt_span_bug_fmt::<rustc_span[85bea1f81190796e]::span_encoding::Span>
  21:     0x7f28bdc884e3 - rustc_middle[1e440ea759c2f365]::util::bug::bug_fmt
  22:     0x7f28bfee08e2 - <rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder>::const_param_out_of_range
  23:     0x7f28bda65d8d - <rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder as rustc_middle[1e440ea759c2f365]::ty::fold::TypeFolder>::fold_const
  24:     0x7f28bda675dc - <&rustc_middle[1e440ea759c2f365]::ty::list::List<rustc_middle[1e440ea759c2f365]::ty::subst::GenericArg> as rustc_middle[1e440ea759c2f365]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder>
  25:     0x7f28bda67f1c - <rustc_middle[1e440ea759c2f365]::ty::consts::Const as rustc_middle[1e440ea759c2f365]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder>
  26:     0x7f28bda63966 - <rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder as rustc_middle[1e440ea759c2f365]::ty::fold::FallibleTypeFolder>::try_fold_ty
  27:     0x7f28bfedf16f - <rustc_middle[1e440ea759c2f365]::ty::subst::GenericArg as rustc_middle[1e440ea759c2f365]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[1e440ea759c2f365]::ty::subst::SubstFolder>
  28:     0x7f28bdaf696c - <rustc_middle[1e440ea759c2f365]::ty::generics::GenericPredicates>::instantiate_into
  29:     0x7f28bdc615fc - <rustc_trait_selection[429aeeaf97ad5f33]::traits::wf::WfPredicates>::nominal_obligations_inner
  30:     0x7f28bdc5de9b - <rustc_trait_selection[429aeeaf97ad5f33]::traits::wf::WfPredicates>::compute
  31:     0x7f28be9164b3 - rustc_trait_selection[429aeeaf97ad5f33]::traits::wf::obligations
  32:     0x7f28be463b35 - <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[5aec1d928cb65a40]::vec::into_iter::IntoIter<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>>
  33:     0x7f28be45e996 - <rustc_infer[eb3d8a9fc270db4a]::infer::InferCtxt>::probe::<core[6f75f940baff7b16]::result::Result<rustc_middle[1e440ea759c2f365]::traits::select::EvaluationResult, rustc_middle[1e440ea759c2f365]::traits::select::OverflowError>, <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  34:     0x7f28be45a6e2 - <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  35:     0x7f28bdc64094 - <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::select
  36:     0x7f28be210789 - rustc_trait_selection[429aeeaf97ad5f33]::traits::project::opt_normalize_projection_type
  37:     0x7f28be1c7824 - <rustc_infer[eb3d8a9fc270db4a]::infer::InferCtxt>::commit_if_ok::<rustc_trait_selection[429aeeaf97ad5f33]::traits::project::ProjectAndUnifyResult, rustc_infer[eb3d8a9fc270db4a]::traits::project::MismatchedProjectionTypes, rustc_trait_selection[429aeeaf97ad5f33]::traits::project::poly_project_and_unify_type::{closure#0}>
  38:     0x7f28be52785d - <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluate_predicate_recursively
  39:     0x7f28bef3372c - <rustc_infer[eb3d8a9fc270db4a]::infer::InferCtxt>::probe::<core[6f75f940baff7b16]::result::Result<rustc_middle[1e440ea759c2f365]::traits::select::EvaluationResult, rustc_middle[1e440ea759c2f365]::traits::select::OverflowError>, <rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[429aeeaf97ad5f33]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  40:     0x7f28bef4b769 - <core[6f75f940baff7b16]::iter::adapters::copied::Copied<core[6f75f940baff7b16]::slice::iter::Iter<rustc_middle[1e440ea759c2f365]::ty::Predicate>> as core[6f75f940baff7b16]::iter::traits::iterator::Iterator>::try_fold::<(), &mut core[6f75f940baff7b16]::iter::adapters::map::map_try_fold<rustc_middle[1e440ea759c2f365]::ty::Predicate, rustc_middle[1e440ea759c2f365]::ty::Predicate, (), core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#0}, core[6f75f940baff7b16]::iter::adapters::map::map_try_fold<rustc_middle[1e440ea759c2f365]::ty::Predicate, rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>, (), core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#1}, core[6f75f940baff7b16]::iter::traits::iterator::Iterator::find::check<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>, &mut rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#2}>::{closure#0}>::{closure#0}>::{closure#0}, core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>>
  41:     0x7f28bef4b503 - <core[6f75f940baff7b16]::iter::adapters::chain::Chain<core[6f75f940baff7b16]::iter::adapters::copied::Copied<core[6f75f940baff7b16]::slice::iter::Iter<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, alloc[5aec1d928cb65a40]::vec::into_iter::IntoIter<rustc_middle[1e440ea759c2f365]::ty::Predicate>> as core[6f75f940baff7b16]::iter::traits::iterator::Iterator>::try_fold::<(), core[6f75f940baff7b16]::iter::adapters::map::map_try_fold<rustc_middle[1e440ea759c2f365]::ty::Predicate, rustc_middle[1e440ea759c2f365]::ty::Predicate, (), core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#0}, core[6f75f940baff7b16]::iter::adapters::map::map_try_fold<rustc_middle[1e440ea759c2f365]::ty::Predicate, rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>, (), core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#1}, core[6f75f940baff7b16]::iter::traits::iterator::Iterator::find::check<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>, &mut rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::implicit_negative::{closure#2}>::{closure#0}>::{closure#0}>::{closure#0}, core[6f75f940baff7b16]::ops::control_flow::ControlFlow<rustc_infer[eb3d8a9fc270db4a]::traits::Obligation<rustc_middle[1e440ea759c2f365]::ty::Predicate>>>
  42:     0x7f28bed8f467 - rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::overlap_within_probe
  43:     0x7f28bed8e8d3 - <rustc_infer[eb3d8a9fc270db4a]::infer::InferCtxt>::probe_maybe_skip_leak_check::<core[6f75f940baff7b16]::option::Option<rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::OverlapResult>, rustc_trait_selection[429aeeaf97ad5f33]::traits::coherence::overlap::{closure#0}>
  44:     0x7f28bed8bb8c - <rustc_middle[1e440ea759c2f365]::traits::specialization_graph::Children as rustc_trait_selection[429aeeaf97ad5f33]::traits::specialize::specialization_graph::ChildrenExt>::insert
  45:     0x7f28bed88cd3 - <rustc_middle[1e440ea759c2f365]::traits::specialization_graph::Graph as rustc_trait_selection[429aeeaf97ad5f33]::traits::specialize::specialization_graph::GraphExt>::insert
  46:     0x7f28bed86e3d - rustc_trait_selection[429aeeaf97ad5f33]::traits::specialize::specialization_graph_provider
  47:     0x7f28bed433f8 - rustc_query_system[9e10ef674197eb88]::query::plumbing::try_execute_query::<rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt, rustc_query_system[9e10ef674197eb88]::query::caches::ArenaCache<rustc_span[85bea1f81190796e]::def_id::DefId, rustc_middle[1e440ea759c2f365]::traits::specialization_graph::Graph>>
  48:     0x7f28becace5e - rustc_query_system[9e10ef674197eb88]::query::plumbing::get_query::<rustc_query_impl[94dddaa624b13a2e]::queries::specialization_graph_of, rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt>
  49:     0x7f28bf07f93b - rustc_hir_analysis[9182be28d676711c]::coherence::coherent_trait
  50:     0x7f28be3c5dc8 - rustc_query_system[9e10ef674197eb88]::query::plumbing::try_execute_query::<rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt, rustc_query_system[9e10ef674197eb88]::query::caches::DefaultCache<rustc_span[85bea1f81190796e]::def_id::DefId, ()>>
  51:     0x7f28bed70c65 - rustc_query_system[9e10ef674197eb88]::query::plumbing::get_query::<rustc_query_impl[94dddaa624b13a2e]::queries::coherent_trait, rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt>
  52:     0x7f28bde36fa2 - <rustc_session[638750f383577479]::session::Session>::track_errors::<rustc_hir_analysis[9182be28d676711c]::check_crate::{closure#3}, ()>
  53:     0x7f28bde33003 - rustc_hir_analysis[9182be28d676711c]::check_crate
  54:     0x7f28bde32cdb - rustc_interface[e10a04643064e0c3]::passes::analysis
  55:     0x7f28bf26ecb4 - rustc_query_system[9e10ef674197eb88]::query::plumbing::try_execute_query::<rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt, rustc_query_system[9e10ef674197eb88]::query::caches::DefaultCache<(), core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>>
  56:     0x7f28bf26e9b7 - rustc_query_system[9e10ef674197eb88]::query::plumbing::get_query::<rustc_query_impl[94dddaa624b13a2e]::queries::analysis, rustc_query_impl[94dddaa624b13a2e]::plumbing::QueryCtxt>
  57:     0x7f28bedb16fd - <rustc_interface[e10a04643064e0c3]::passes::QueryContext>::enter::<rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  58:     0x7f28bedadbaf - <rustc_interface[e10a04643064e0c3]::interface::Compiler>::enter::<rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}::{closure#2}, core[6f75f940baff7b16]::result::Result<core[6f75f940baff7b16]::option::Option<rustc_interface[e10a04643064e0c3]::queries::Linker>, rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  59:     0x7f28beda56f2 - rustc_span[85bea1f81190796e]::with_source_map::<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  60:     0x7f28beda51e9 - <scoped_tls[df6a59750083ca9d]::ScopedKey<rustc_span[85bea1f81190796e]::SessionGlobals>>::set::<rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  61:     0x7f28beda47f8 - std[6a238fb43aa4c274]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e10a04643064e0c3]::util::run_in_thread_pool_with_globals<rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>
  62:     0x7f28beda451c - <<std[6a238fb43aa4c274]::thread::Builder>::spawn_unchecked_<rustc_interface[e10a04643064e0c3]::util::run_in_thread_pool_with_globals<rustc_interface[e10a04643064e0c3]::interface::run_compiler<core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>, rustc_driver[2731657a57c33a9f]::run_compiler::{closure#1}>::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6f75f940baff7b16]::result::Result<(), rustc_errors[8289f13c10dc8783]::ErrorGuaranteed>>::{closure#1} as core[6f75f940baff7b16]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  63:     0x7f28c0747bc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h23aa6e7db304ed51
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/alloc/src/boxed.rs:1987:9
  64:     0x7f28c0747bc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0482f1835c06f38d
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/alloc/src/boxed.rs:1987:9
  65:     0x7f28c0747bc3 - std::sys::unix::thread::Thread::new::thread_start::h5213f0bce91e8f3e
                               at /rustc/e080cc5a659fb760c0bc561b722a790dad35b5e1/library/std/src/sys/unix/thread.rs:108:17
  66:     0x7f28bc30f6db - start_thread
  67:     0x7f28bc03861f - clone
  68:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-beta.1 (e080cc5a6 2022-11-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [specialization_graph_of] building specialization graph of trait `collections::SpecExtend`
#1 [coherent_trait] coherence checking all impls of trait `collections::SpecExtend`
#2 [analysis] running analysis passes on this crate
error: could not compile `alloc`
Build completed unsuccessfully in 0:02:31
