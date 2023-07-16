plain
---- [rustdoc] tests/rustdoc/intra-doc/libstd-re-export.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export" "--deny" "warnings" "/checkout/tests/rustdoc/intra-doc/libstd-re-export.rs"
stdout: none
--- stderr -------------------------------
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/fast_reject.rs:366:17: unexpected impl arg: Const { ty: usize, kind: Infer(Var(?0c)) }
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
stack backtrace:
   0:     0x7fdabc8a3434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h672970d47dc0dc37
   1:     0x7fdabc909ff8 - core::fmt::write::hc52ac4db98ecefd2
   2:     0x7fdabc897b41 - std::io::Write::write_fmt::h9b0ebd07ab448b2c
   3:     0x7fdabc8a3241 - std::sys_common::backtrace::print::h1e294cf4b3afa248
   4:     0x7fdabc8a63ba - std::panicking::default_hook::{{closure}}::hc359736cd11fc1e0
   5:     0x7fdabc8a609c - std::panicking::default_hook::h24555a10a6c152cd
   6:     0x7fdabd37490b - rustc_driver_impl[e8f667e2b5ac37bc]::install_ice_hook::{closure#0}
   7:     0x7fdabc8a6ac7 - std::panicking::rust_panic_with_hook::h1260f668b2f85983
   8:     0x7fdac000d373 - std[f16136f58ef6b057]::panicking::begin_panic::<rustc_errors[8992f15653b261ea]::ExplicitBug>::{closure#0}
   9:     0x7fdac00086c6 - std[f16136f58ef6b057]::sys_common::backtrace::__rust_end_short_backtrace::<std[f16136f58ef6b057]::panicking::begin_panic<rustc_errors[8992f15653b261ea]::ExplicitBug>::{closure#0}, !>
  10:     0x7fdabd30d616 - std[f16136f58ef6b057]::panicking::begin_panic::<rustc_errors[8992f15653b261ea]::ExplicitBug>
  11:     0x7fdac0000d67 - <rustc_errors[8992f15653b261ea]::HandlerInner>::bug::<alloc[13b33331e49a9486]::string::String>
  12:     0x7fdac0000a39 - <rustc_errors[8992f15653b261ea]::Handler>::bug::<alloc[13b33331e49a9486]::string::String>
  13:     0x7fdac01b7307 - rustc_middle[734e3452d1224342]::util::bug::opt_span_bug_fmt::<rustc_span[b8b6744662c46af5]::span_encoding::Span>::{closure#0}
  14:     0x7fdac01b5e5c - rustc_middle[734e3452d1224342]::ty::context::tls::with_opt::<rustc_middle[734e3452d1224342]::util::bug::opt_span_bug_fmt<rustc_span[b8b6744662c46af5]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7fdac01b5de4 - rustc_middle[734e3452d1224342]::ty::context::tls::with_context_opt::<rustc_middle[734e3452d1224342]::ty::context::tls::with_opt<rustc_middle[734e3452d1224342]::util::bug::opt_span_bug_fmt<rustc_span[b8b6744662c46af5]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7fdabd316ca2 - rustc_middle[734e3452d1224342]::util::bug::bug_fmt
  17:     0x7fdabffa772e - <rustc_middle[734e3452d1224342]::ty::fast_reject::DeepRejectCtxt>::consts_may_unify
  18:     0x7fdabffa7203 - <rustc_middle[734e3452d1224342]::ty::fast_reject::DeepRejectCtxt>::substs_refs_may_unify
  19:     0x7fdabffa71d7 - <rustc_middle[734e3452d1224342]::ty::fast_reject::DeepRejectCtxt>::substs_refs_may_unify
  20:     0x7fdabfd51670 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::assemble_candidates_from_caller_bounds
  21:     0x7fdabfd4fe89 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::assemble_candidates
  22:     0x7fdabfd47c20 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  23:     0x7fdabfbf2bbb - <rustc_query_system[582a77f0b9c1a89b]::dep_graph::graph::DepGraph<rustc_middle[734e3452d1224342]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[734e3452d1224342]::ty::context::TyCtxt, <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::candidate_from_obligation::{closure#0}::{closure#0}, core[f9b88b19e6e7a299]::result::Result<core[f9b88b19e6e7a299]::option::Option<rustc_middle[734e3452d1224342]::traits::select::SelectionCandidate>, rustc_middle[734e3452d1224342]::traits::SelectionError>>::{closure#0}, core[f9b88b19e6e7a299]::result::Result<core[f9b88b19e6e7a299]::option::Option<rustc_middle[734e3452d1224342]::traits::select::SelectionCandidate>, rustc_middle[734e3452d1224342]::traits::SelectionError>>
  24:     0x7fdabfd631a4 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::candidate_from_obligation
  25:     0x7fdabfd4c17d - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_stack
  26:     0x7fdabfbf33d3 - <rustc_query_system[582a77f0b9c1a89b]::dep_graph::graph::DepGraph<rustc_middle[734e3452d1224342]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[734e3452d1224342]::ty::context::TyCtxt, <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[f9b88b19e6e7a299]::result::Result<rustc_middle[734e3452d1224342]::traits::select::EvaluationResult, rustc_middle[734e3452d1224342]::traits::select::OverflowError>>::{closure#0}, core[f9b88b19e6e7a299]::result::Result<rustc_middle[734e3452d1224342]::traits::select::EvaluationResult, rustc_middle[734e3452d1224342]::traits::select::OverflowError>>
  27:     0x7fdabfd69024 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  28:     0x7fdabfd667d0 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_predicate_recursively
  29:     0x7fdabfd65e85 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[13b33331e49a9486]::vec::into_iter::IntoIter<rustc_infer[f8b06100f53e6f82]::traits::Obligation<rustc_middle[734e3452d1224342]::ty::Predicate>>>
  30:     0x7fdabfcdd582 - <rustc_infer[f8b06100f53e6f82]::infer::InferCtxt>::probe::<core[f9b88b19e6e7a299]::result::Result<rustc_middle[734e3452d1224342]::traits::select::EvaluationResult, rustc_middle[734e3452d1224342]::traits::select::OverflowError>, <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  31:     0x7fdabfd6b946 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_candidate
  32:     0x7fdabfd4c1e5 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_stack
  33:     0x7fdabfbf33d3 - <rustc_query_system[582a77f0b9c1a89b]::dep_graph::graph::DepGraph<rustc_middle[734e3452d1224342]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[734e3452d1224342]::ty::context::TyCtxt, <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[f9b88b19e6e7a299]::result::Result<rustc_middle[734e3452d1224342]::traits::select::EvaluationResult, rustc_middle[734e3452d1224342]::traits::select::OverflowError>>::{closure#0}, core[f9b88b19e6e7a299]::result::Result<rustc_middle[734e3452d1224342]::traits::select::EvaluationResult, rustc_middle[734e3452d1224342]::traits::select::OverflowError>>
  34:     0x7fdabfd69024 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  35:     0x7fdabfd667d0 - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_predicate_recursively
  36:     0x7fdabfcde050 - <rustc_infer[f8b06100f53e6f82]::infer::InferCtxt>::probe::<core[f9b88b19e6e7a299]::result::Result<rustc_middle[734e3452d1224342]::traits::select::EvaluationResult, rustc_middle[734e3452d1224342]::traits::select::OverflowError>, <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  37:     0x7fdabfd4bd2c - <rustc_trait_selection[69942c6c986dfc38]::traits::select::SelectionContext>::evaluate_root_obligation
  38:     0x7fdabeb04f12 - rustc_traits[aeec32cddbf49cb8]::evaluate_obligation::evaluate_obligation
  39:     0x7fdabf088123 - rustc_query_system[582a77f0b9c1a89b]::query::plumbing::try_execute_query::<rustc_query_impl[d81a8dee24f5e0bc]::queries::evaluate_obligation, rustc_query_impl[d81a8dee24f5e0bc]::plumbing::QueryCtxt>
  40:     0x7fdabef1ed72 - rustc_query_impl[d81a8dee24f5e0bc]::get_query::evaluate_obligation
  41:     0x7fdabfced489 - <rustc_infer[f8b06100f53e6f82]::infer::InferCtxt as rustc_trait_selection[69942c6c986dfc38]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  42:     0x55840d54ce63 - <rustdoc[3e0251304c73c236]::clean::blanket_impl::BlanketImplFinder>::get_blanket_impls
  43:     0x55840d258647 - rustdoc[3e0251304c73c236]::clean::utils::get_auto_trait_and_blanket_impls
  44:     0x55840d4d53ad - <rustdoc[3e0251304c73c236]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[3e0251304c73c236]::visit::DocVisitor>::visit_item
  45:     0x55840d4d556a - <rustdoc[3e0251304c73c236]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[3e0251304c73c236]::visit::DocVisitor>::visit_item
  46:     0x55840d4d556a - <rustdoc[3e0251304c73c236]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[3e0251304c73c236]::visit::DocVisitor>::visit_item
  47:     0x55840d4d60c1 - <rustdoc[3e0251304c73c236]::passes::collect_trait_impls::SyntheticImplCollector as rustdoc[3e0251304c73c236]::visit::DocVisitor>::visit_crate
  48:     0x55840d279468 - <rustc_session[3724e9ffb9c2c75b]::session::Session>::time::<alloc[13b33331e49a9486]::vec::Vec<rustdoc[3e0251304c73c236]::clean::types::Item>, rustdoc[3e0251304c73c236]::passes::collect_trait_impls::collect_trait_impls::{closure#0}>
  49:     0x55840d4d3b3a - rustdoc[3e0251304c73c236]::passes::collect_trait_impls::collect_trait_impls
  50:     0x55840d279c41 - <rustc_session[3724e9ffb9c2c75b]::session::Session>::time::<rustdoc[3e0251304c73c236]::clean::types::Crate, rustdoc[3e0251304c73c236]::core::run_global_ctxt::{closure#8}>
  51:     0x55840d388076 - rustdoc[3e0251304c73c236]::core::run_global_ctxt
  52:     0x55840d27b4a3 - <rustc_session[3724e9ffb9c2c75b]::session::Session>::time::<(rustdoc[3e0251304c73c236]::clean::types::Crate, rustdoc[3e0251304c73c236]::config::RenderOptions, rustdoc[3e0251304c73c236]::formats::cache::Cache), rustdoc[3e0251304c73c236]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  53:     0x55840d3bc42b - <rustc_middle[734e3452d1224342]::ty::context::GlobalCtxt>::enter::<rustdoc[3e0251304c73c236]::main_args::{closure#1}::{closure#0}::{closure#0}, core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>>
  54:     0x55840d2f927c - <rustc_interface[e5ddf8b6b7e8be6b]::queries::QueryResult<&rustc_middle[734e3452d1224342]::ty::context::GlobalCtxt>>::enter::<core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>, rustdoc[3e0251304c73c236]::main_args::{closure#1}::{closure#0}::{closure#0}>
  55:     0x55840d46297d - <rustc_interface[e5ddf8b6b7e8be6b]::interface::Compiler>::enter::<rustdoc[3e0251304c73c236]::main_args::{closure#1}::{closure#0}, core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>>
  56:     0x55840d33dbe9 - <scoped_tls[eb838f9b5f0bb653]::ScopedKey<rustc_span[b8b6744662c46af5]::SessionGlobals>>::set::<rustc_interface[e5ddf8b6b7e8be6b]::interface::run_compiler<core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>, rustdoc[3e0251304c73c236]::main_args::{closure#1}>::{closure#0}, core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>>
  57:     0x55840d230e46 - std[f16136f58ef6b057]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e5ddf8b6b7e8be6b]::util::run_in_thread_pool_with_globals<rustc_interface[e5ddf8b6b7e8be6b]::interface::run_compiler<core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>, rustdoc[3e0251304c73c236]::main_args::{closure#1}>::{closure#0}, core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>>
  58:     0x55840d30df19 - <<std[f16136f58ef6b057]::thread::Builder>::spawn_unchecked_<rustc_interface[e5ddf8b6b7e8be6b]::util::run_in_thread_pool_with_globals<rustc_interface[e5ddf8b6b7e8be6b]::interface::run_compiler<core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>, rustdoc[3e0251304c73c236]::main_args::{closure#1}>::{closure#0}, core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f9b88b19e6e7a299]::result::Result<(), rustc_span[b8b6744662c46af5]::ErrorGuaranteed>>::{closure#1} as core[f9b88b19e6e7a299]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7fdabc8b33fe - std::sys::unix::thread::Thread::new::thread_start::h7d44cd08d4a6b35e
  60:     0x7fdabc545b43 - <unknown>
  61:     0x7fdabc5d7a00 - <unknown>
  62:                0x0 - <unknown>

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-rustdoc&template=ice.md
note: rustc 1.71.0-nightly (90b2583c8 2023-05-11) running on x86_64-unknown-linux-gnu

query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `core::core_simd::vector::Simd<^1_1, ^1_0>: core::clone::Clone`
error: aborting due to previous error
------------------------------------------


