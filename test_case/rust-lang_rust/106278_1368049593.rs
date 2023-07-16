plain
........................................................................................ 352/14093
........................................................................................ 440/14093
........................................................................................ 528/14093
........................................................................................ 616/14093
......F..F.F............................................................................ 704/14093
F.................F..........................F.......................................F.. 792/14093
.F..............F......................................i................................ 880/14093
........................................................................................ 1056/14093
........................................................................................ 1144/14093
............F........................................................................... 1232/14093
........................................................................................ 1320/14093
---
failures:

---- [ui] src/test/ui/async-await/async-fn-nonsend.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/auxiliary" "--edition=2018" "--crate-type" "lib" "-Zdrop-tracking"
stdout: none
--- stderr -------------------------------
  left: `0`,
 right: `3`', /checkout/compiler/rustc_middle/src/ty/relate.rs:355:9
stack backtrace:
   0:     0x7f261b375175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3654c56af00a5f5e
   0:     0x7f261b375175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3654c56af00a5f5e
   1:     0x7f261b3e51e8 - core::fmt::write::h2094bcbce382bdd5
   2:     0x7f261b367291 - std::io::Write::write_fmt::h8656690cbcb421cc
   3:     0x7f261b374f81 - std::sys_common::backtrace::print::hb2648e0ef907ed83
   4:     0x7f261b378364 - std::panicking::default_hook::{{closure}}::h57dad3e290d67faf
   5:     0x7f261b37802a - std::panicking::default_hook::h7de156a9f4432d67
   6:     0x7f261bdc5392 - rustc_driver[5180676712772da4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f261b378ad4 - std::panicking::rust_panic_with_hook::h189d11e83e0d3c1d
   8:     0x7f261b378839 - std::panicking::begin_panic_handler::{{closure}}::h9c14fa00d7507409
   9:     0x7f261b375694 - std::sys_common::backtrace::__rust_end_short_backtrace::hecf0b7131e816200
  10:     0x7f261b3784e2 - rust_begin_unwind
  11:     0x7f261b328fc3 - core::panicking::panic_fmt::hffb5048cd1767ea0
  12:     0x7f261b32932c - core::panicking::assert_failed_inner::h220e49d01e68d450
  13:     0x7f261bd3c7eb - core[9d5edb83b0e1b9c4]::panicking::assert_failed::<usize, usize>
  14:     0x7f261e786ac1 - <rustc_middle[79b39f412ff7aebc]::ty::relate::GeneratorWitness as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  15:     0x7f261e7c97c0 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::binders::<rustc_middle[79b39f412ff7aebc]::ty::relate::GeneratorWitness>
  16:     0x7f261e7753bc - rustc_middle[79b39f412ff7aebc]::ty::relate::super_relate_tys::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  17:     0x7f261e7d8863 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::tys
  18:     0x7f261e8715ee - <rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  19:     0x7f261e789d9c - <&mut rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0} as core[9d5edb83b0e1b9c4]::ops::function::FnOnce<((rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg),)>>::call_once
  20:     0x7f261e7b159b - <core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, rustc_middle[79b39f412ff7aebc]::ty::error::TypeError> as rustc_type_ir[c54a98da0e3bcbbf]::InternIteratorElement<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, &rustc_middle[79b39f412ff7aebc]::ty::list::List<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>::intern_with::<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>, <rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt>::mk_substs<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>>::{closure#0}>
  21:     0x7f261e86908f - <rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt>::mk_substs::<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>>
  22:     0x7f261e6b6b28 - <rustc_middle[79b39f412ff7aebc]::ty::sty::TraitRef as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  23:     0x7f261e7b8414 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::relate::<rustc_middle[79b39f412ff7aebc]::ty::sty::TraitRef>
  24:     0x7f261e88a0b6 - <rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  25:     0x7f261e7c98b6 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::binders::<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate>
  26:     0x7f261e6bba26 - <rustc_middle[79b39f412ff7aebc]::ty::sty::Binder<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate> as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  27:     0x7f261e7b8236 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::relate::<rustc_middle[79b39f412ff7aebc]::ty::sty::Binder<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate>>
  28:     0x7f261e838ce2 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  29:     0x7f261e6d0c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  30:     0x7f261e8564b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  31:     0x7f261e854001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  32:     0x7f261e853775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  33:     0x7f261e75fed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  34:     0x7f261e85926a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  35:     0x7f261e838fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  36:     0x7f261e6d0c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  37:     0x7f261e8564b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  38:     0x7f261e854001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  39:     0x7f261e853775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  40:     0x7f261e75fed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  41:     0x7f261e85926a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  42:     0x7f261e838fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  43:     0x7f261e6d0c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  44:     0x7f261e8564b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  45:     0x7f261e854001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  46:     0x7f261e853775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  47:     0x7f261e75fed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  48:     0x7f261e85926a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  49:     0x7f261e838fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  50:     0x7f261e6d0c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  51:     0x7f261e8564b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  52:     0x7f261e854001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  53:     0x7f261e853775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  54:     0x7f261e75fed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  55:     0x7f261e85926a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  56:     0x7f261e838fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  57:     0x7f261e6d0c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  58:     0x7f261e8564b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  59:     0x7f261e854001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  60:     0x7f261e853775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  61:     0x7f261e75fed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  62:     0x7f261e85926a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  63:     0x7f261e838fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  64:     0x7f261e6d0c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  65:     0x7f261e8564b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  66:     0x7f261e854001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  67:     0x7f261e760993 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  68:     0x7f261e838bcd - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_root_obligation
  69:     0x7f261d429edf - rustc_traits[9e65619632aadd08]::evaluate_obligation::evaluate_obligation
  70:     0x7f261da477b3 - <rustc_query_system[9dff316cef8173b0]::query::config::QueryVTable<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::infer::canonical::Canonical<rustc_middle[79b39f412ff7aebc]::ty::ParamEnvAnd<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>>::compute
  71:     0x7f261dcb8c05 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::evaluate_obligation, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
  72:     0x7f261d840058 - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::evaluate_obligation
  73:     0x7f261e76f2e6 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt as rustc_trait_selection[53160d62df8e776c]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  74:     0x7f261e76f48c - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt as rustc_trait_selection[53160d62df8e776c]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  75:     0x7f261e8818c3 - <rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  76:     0x7f261e87fbf8 - <rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillProcessor as rustc_data_structures[27a7519d22327df0]::obligation_forest::ObligationProcessor>::process_obligation
  77:     0x7f261e793d6d - <rustc_data_structures[27a7519d22327df0]::obligation_forest::ObligationForest<rustc_trait_selection[53160d62df8e776c]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillProcessor>
  78:     0x7f261e87df07 - <rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillmentContext as rustc_infer[d164ecb974ac35ed]::traits::engine::TraitEngine>::select_where_possible
  79:     0x7f261c2efd96 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_argument_types
  80:     0x7f261c2bf5a4 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::confirm_builtin_call
  81:     0x7f261c2be3fa - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_call
  82:     0x7f261c342c62 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_kind
  83:     0x7f261c2d5ab1 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  84:     0x7f261c341b42 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  85:     0x7f261c2f7941 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_stmt
  86:     0x7f261c2f7ea7 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_block_with_expected
  87:     0x7f261c342fd8 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_kind
  88:     0x7f261c2d5ab1 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  89:     0x7f261c341b42 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  90:     0x7f261c2d7fbb - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_return_expr
  91:     0x7f261c4d0805 - rustc_hir_typeck[97d777f68f886a7c]::check::check_fn
  92:     0x7f261c4062fb - <rustc_hir_typeck[97d777f68f886a7c]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[97d777f68f886a7c]::typeck_with_fallback<rustc_hir_typeck[97d777f68f886a7c]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[79b39f412ff7aebc]::ty::typeck_results::TypeckResults>
  93:     0x7f261c427abf - rustc_hir_typeck[97d777f68f886a7c]::typeck
  94:     0x7f261dbff243 - rustc_query_system[9dff316cef8173b0]::query::plumbing::try_execute_query::<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_query_system[9dff316cef8173b0]::query::caches::VecCache<rustc_span[fe3b109d6a205fd9]::def_id::LocalDefId, &rustc_middle[79b39f412ff7aebc]::ty::typeck_results::TypeckResults>>
  95:     0x7f261dcfa3f6 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::typeck, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
  96:     0x7f261d7f8090 - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::typeck
  97:     0x7f261c3991d4 - std[ffda4b28d2e2b2d5]::panicking::try::<(), core[9d5edb83b0e1b9c4]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[27a7519d22327df0]::sync::par_for_each_in<&[rustc_span[fe3b109d6a205fd9]::def_id::LocalDefId], <rustc_middle[79b39f412ff7aebc]::hir::map::Map>::par_body_owners<rustc_hir_typeck[97d777f68f886a7c]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  98:     0x7f261c3970ed - rustc_data_structures[27a7519d22327df0]::sync::par_for_each_in::<&[rustc_span[fe3b109d6a205fd9]::def_id::LocalDefId], <rustc_middle[79b39f412ff7aebc]::hir::map::Map>::par_body_owners<rustc_hir_typeck[97d777f68f886a7c]::typeck_item_bodies::{closure#0}>::{closure#0}>
  99:     0x7f261c426a97 - rustc_hir_typeck[97d777f68f886a7c]::typeck_item_bodies
 100:     0x7f261dbd421e - rustc_query_system[9dff316cef8173b0]::query::plumbing::try_execute_query::<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_query_system[9dff316cef8173b0]::query::caches::DefaultCache<(), ()>>
 101:     0x7f261dcb7cf4 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::typeck_item_bodies, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
 102:     0x7f261d7f792a - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::typeck_item_bodies
 103:     0x7f261c60006b - <rustc_session[ab216781b4852cbf]::session::Session>::time::<(), rustc_hir_analysis[51301afa03c42b59]::check_crate::{closure#7}>
 104:     0x7f261c582f23 - rustc_hir_analysis[51301afa03c42b59]::check_crate
 105:     0x7f261bf21921 - rustc_interface[a07a1f0e4c3f1918]::passes::analysis
 106:     0x7f261dbc4d1b - rustc_query_system[9dff316cef8173b0]::query::plumbing::try_execute_query::<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_query_system[9dff316cef8173b0]::query::caches::DefaultCache<(), core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>>
 107:     0x7f261dcfa7d4 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::analysis, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
 108:     0x7f261d7cefba - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::analysis
 109:     0x7f261be22e1c - <rustc_interface[a07a1f0e4c3f1918]::passes::QueryContext>::enter::<rustc_driver[5180676712772da4]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 110:     0x7f261be2c9ba - <rustc_interface[a07a1f0e4c3f1918]::interface::Compiler>::enter::<rustc_driver[5180676712772da4]::run_compiler::{closure#1}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<core[9d5edb83b0e1b9c4]::option::Option<rustc_interface[a07a1f0e4c3f1918]::queries::Linker>, rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 111:     0x7f261bdc6afc - rustc_span[fe3b109d6a205fd9]::with_source_map::<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
 112:     0x7f261be21cfa - <scoped_tls[3671739d64d07b67]::ScopedKey<rustc_span[fe3b109d6a205fd9]::SessionGlobals>>::set::<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 113:     0x7f261bde4caf - std[ffda4b28d2e2b2d5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a07a1f0e4c3f1918]::util::run_in_thread_pool_with_globals<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 114:     0x7f261be27f76 - std[ffda4b28d2e2b2d5]::panic::catch_unwind::<core[9d5edb83b0e1b9c4]::panic::unwind_safe::AssertUnwindSafe<<std[ffda4b28d2e2b2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[a07a1f0e4c3f1918]::util::run_in_thread_pool_with_globals<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 115:     0x7f261bdd71b5 - <<std[ffda4b28d2e2b2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[a07a1f0e4c3f1918]::util::run_in_thread_pool_with_globals<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#1} as core[9d5edb83b0e1b9c4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 116:     0x7f261b3858ce - std::sys::unix::thread::Thread::new::thread_start::hd5e033cd96f77265
 117:     0x7f261b11ab43 - <unknown>
 118:     0x7f261b1aca00 - <unknown>
 119:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (d83a85bc3 2022-12-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib -Z drop-tracking
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Send`
#1 [typeck] type-checking `pass_assert`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/async-await/async-is-unwindsafe.rs stdout ----
---- [ui] src/test/ui/async-await/async-is-unwindsafe.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-is-unwindsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-is-unwindsafe" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-is-unwindsafe/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  left: `0`,
 right: `3`', /checkout/compiler/rustc_middle/src/ty/relate.rs:355:9
stack backtrace:
   0:     0x7f5424354175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3654c56af00a5f5e
   0:     0x7f5424354175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3654c56af00a5f5e
   1:     0x7f54243c41e8 - core::fmt::write::h2094bcbce382bdd5
   2:     0x7f5424346291 - std::io::Write::write_fmt::h8656690cbcb421cc
   3:     0x7f5424353f81 - std::sys_common::backtrace::print::hb2648e0ef907ed83
   4:     0x7f5424357364 - std::panicking::default_hook::{{closure}}::h57dad3e290d67faf
   5:     0x7f542435702a - std::panicking::default_hook::h7de156a9f4432d67
   6:     0x7f5424da4392 - rustc_driver[5180676712772da4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5424357ad4 - std::panicking::rust_panic_with_hook::h189d11e83e0d3c1d
   8:     0x7f5424357839 - std::panicking::begin_panic_handler::{{closure}}::h9c14fa00d7507409
   9:     0x7f5424354694 - std::sys_common::backtrace::__rust_end_short_backtrace::hecf0b7131e816200
  10:     0x7f54243574e2 - rust_begin_unwind
  11:     0x7f5424307fc3 - core::panicking::panic_fmt::hffb5048cd1767ea0
  12:     0x7f542430832c - core::panicking::assert_failed_inner::h220e49d01e68d450
  13:     0x7f5424d1b7eb - core[9d5edb83b0e1b9c4]::panicking::assert_failed::<usize, usize>
  14:     0x7f5427765ac1 - <rustc_middle[79b39f412ff7aebc]::ty::relate::GeneratorWitness as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  15:     0x7f54277a87c0 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::binders::<rustc_middle[79b39f412ff7aebc]::ty::relate::GeneratorWitness>
  16:     0x7f54277543bc - rustc_middle[79b39f412ff7aebc]::ty::relate::super_relate_tys::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  17:     0x7f54277b7863 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::tys
  18:     0x7f54278505ee - <rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  19:     0x7f5427768d9c - <&mut rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0} as core[9d5edb83b0e1b9c4]::ops::function::FnOnce<((rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg),)>>::call_once
  20:     0x7f542779059b - <core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, rustc_middle[79b39f412ff7aebc]::ty::error::TypeError> as rustc_type_ir[c54a98da0e3bcbbf]::InternIteratorElement<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, &rustc_middle[79b39f412ff7aebc]::ty::list::List<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>::intern_with::<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>, <rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt>::mk_substs<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>>::{closure#0}>
  21:     0x7f542784808f - <rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt>::mk_substs::<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>>
  22:     0x7f5427695b28 - <rustc_middle[79b39f412ff7aebc]::ty::sty::TraitRef as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  23:     0x7f5427797414 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::relate::<rustc_middle[79b39f412ff7aebc]::ty::sty::TraitRef>
  24:     0x7f54278690b6 - <rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  25:     0x7f54277a88b6 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::binders::<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate>
  26:     0x7f542769aa26 - <rustc_middle[79b39f412ff7aebc]::ty::sty::Binder<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate> as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  27:     0x7f5427797236 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::relate::<rustc_middle[79b39f412ff7aebc]::ty::sty::Binder<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate>>
  28:     0x7f5427817ce2 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  29:     0x7f54276afc11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  30:     0x7f54278354b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  31:     0x7f5427833001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  32:     0x7f5427832775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  33:     0x7f542773eed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  34:     0x7f542783826a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  35:     0x7f5427817fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  36:     0x7f54276afc11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  37:     0x7f54278354b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  38:     0x7f5427833001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  39:     0x7f5427832775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  40:     0x7f542773eed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  41:     0x7f542783826a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  42:     0x7f5427817fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  43:     0x7f54276afc11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  44:     0x7f54278354b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  45:     0x7f5427833001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  46:     0x7f542773f993 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  47:     0x7f5427817bcd - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_root_obligation
  48:     0x7f5426408edf - rustc_traits[9e65619632aadd08]::evaluate_obligation::evaluate_obligation
  49:     0x7f5426a267b3 - <rustc_query_system[9dff316cef8173b0]::query::config::QueryVTable<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::infer::canonical::Canonical<rustc_middle[79b39f412ff7aebc]::ty::ParamEnvAnd<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>>::compute
  50:     0x7f5426c97c05 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::evaluate_obligation, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
  51:     0x7f542681f058 - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::evaluate_obligation
  52:     0x7f542774e2e6 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt as rustc_trait_selection[53160d62df8e776c]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  53:     0x7f542774e48c - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt as rustc_trait_selection[53160d62df8e776c]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  54:     0x7f54278608c3 - <rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  55:     0x7f542785ebf8 - <rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillProcessor as rustc_data_structures[27a7519d22327df0]::obligation_forest::ObligationProcessor>::process_obligation
  56:     0x7f5427772d6d - <rustc_data_structures[27a7519d22327df0]::obligation_forest::ObligationForest<rustc_trait_selection[53160d62df8e776c]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillProcessor>
  57:     0x7f542785cf07 - <rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillmentContext as rustc_infer[d164ecb974ac35ed]::traits::engine::TraitEngine>::select_where_possible
  58:     0x7f54252ca424 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::resolve_generator_interiors
  59:     0x7f54253e55dd - <rustc_hir_typeck[97d777f68f886a7c]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[97d777f68f886a7c]::typeck_with_fallback<rustc_hir_typeck[97d777f68f886a7c]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[79b39f412ff7aebc]::ty::typeck_results::TypeckResults>
  60:     0x7f5425406abf - rustc_hir_typeck[97d777f68f886a7c]::typeck
  61:     0x7f5426bde243 - rustc_query_system[9dff316cef8173b0]::query::plumbing::try_execute_query::<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_query_system[9dff316cef8173b0]::query::caches::VecCache<rustc_span[fe3b109d6a205fd9]::def_id::LocalDefId, &rustc_middle[79b39f412ff7aebc]::ty::typeck_results::TypeckResults>>
  62:     0x7f5426cd93f6 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::typeck, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
  63:     0x7f54267d7090 - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::typeck
  64:     0x7f54253781d4 - std[ffda4b28d2e2b2d5]::panicking::try::<(), core[9d5edb83b0e1b9c4]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[27a7519d22327df0]::sync::par_for_each_in<&[rustc_span[fe3b109d6a205fd9]::def_id::LocalDefId], <rustc_middle[79b39f412ff7aebc]::hir::map::Map>::par_body_owners<rustc_hir_typeck[97d777f68f886a7c]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  65:     0x7f54253760ed - rustc_data_structures[27a7519d22327df0]::sync::par_for_each_in::<&[rustc_span[fe3b109d6a205fd9]::def_id::LocalDefId], <rustc_middle[79b39f412ff7aebc]::hir::map::Map>::par_body_owners<rustc_hir_typeck[97d777f68f886a7c]::typeck_item_bodies::{closure#0}>::{closure#0}>
  66:     0x7f5425405a97 - rustc_hir_typeck[97d777f68f886a7c]::typeck_item_bodies
  67:     0x7f5426bb321e - rustc_query_system[9dff316cef8173b0]::query::plumbing::try_execute_query::<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_query_system[9dff316cef8173b0]::query::caches::DefaultCache<(), ()>>
  68:     0x7f5426c96cf4 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::typeck_item_bodies, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
  69:     0x7f54267d692a - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::typeck_item_bodies
  70:     0x7f54255df06b - <rustc_session[ab216781b4852cbf]::session::Session>::time::<(), rustc_hir_analysis[51301afa03c42b59]::check_crate::{closure#7}>
  71:     0x7f5425561f23 - rustc_hir_analysis[51301afa03c42b59]::check_crate
  72:     0x7f5424f00921 - rustc_interface[a07a1f0e4c3f1918]::passes::analysis
  73:     0x7f5426ba3d1b - rustc_query_system[9dff316cef8173b0]::query::plumbing::try_execute_query::<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_query_system[9dff316cef8173b0]::query::caches::DefaultCache<(), core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>>
  74:     0x7f5426cd97d4 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::analysis, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
  75:     0x7f54267adfba - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::analysis
  76:     0x7f5424e01e1c - <rustc_interface[a07a1f0e4c3f1918]::passes::QueryContext>::enter::<rustc_driver[5180676712772da4]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
  77:     0x7f5424e0b9ba - <rustc_interface[a07a1f0e4c3f1918]::interface::Compiler>::enter::<rustc_driver[5180676712772da4]::run_compiler::{closure#1}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<core[9d5edb83b0e1b9c4]::option::Option<rustc_interface[a07a1f0e4c3f1918]::queries::Linker>, rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
  78:     0x7f5424da5afc - rustc_span[fe3b109d6a205fd9]::with_source_map::<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  79:     0x7f5424e00cfa - <scoped_tls[3671739d64d07b67]::ScopedKey<rustc_span[fe3b109d6a205fd9]::SessionGlobals>>::set::<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
  80:     0x7f5424dc3caf - std[ffda4b28d2e2b2d5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a07a1f0e4c3f1918]::util::run_in_thread_pool_with_globals<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
  81:     0x7f5424e06f76 - std[ffda4b28d2e2b2d5]::panic::catch_unwind::<core[9d5edb83b0e1b9c4]::panic::unwind_safe::AssertUnwindSafe<<std[ffda4b28d2e2b2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[a07a1f0e4c3f1918]::util::run_in_thread_pool_with_globals<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
  82:     0x7f5424db61b5 - <<std[ffda4b28d2e2b2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[a07a1f0e4c3f1918]::util::run_in_thread_pool_with_globals<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#1} as core[9d5edb83b0e1b9c4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  83:     0x7f54243648ce - std::sys::unix::thread::Thread::new::thread_start::hd5e033cd96f77265
  84:     0x7f54240f9b43 - <unknown>
  85:     0x7f542418ba00 - <unknown>
  86:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (d83a85bc3 2022-12-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `{core::future::ResumeTy, [async block@/checkout/src/test/ui/async-await/async-is-unwindsafe.rs:9:9: 9:17], ()}: core::panic::unwind_safe::UnwindSafe`
#1 [typeck] type-checking `main`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/async-await/async-fn-send-uses-nonsend.rs stdout ----
---- [ui] src/test/ui/async-await/async-fn-send-uses-nonsend.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-send-uses-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend/auxiliary" "--edition=2018" "--crate-type" "lib"
stdout: none
--- stderr -------------------------------
  left: `0`,
 right: `7`', /checkout/compiler/rustc_middle/src/ty/relate.rs:355:9
stack backtrace:
   0:     0x7f664b137175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3654c56af00a5f5e
   0:     0x7f664b137175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3654c56af00a5f5e
   1:     0x7f664b1a71e8 - core::fmt::write::h2094bcbce382bdd5
   2:     0x7f664b129291 - std::io::Write::write_fmt::h8656690cbcb421cc
   3:     0x7f664b136f81 - std::sys_common::backtrace::print::hb2648e0ef907ed83
   4:     0x7f664b13a364 - std::panicking::default_hook::{{closure}}::h57dad3e290d67faf
   5:     0x7f664b13a02a - std::panicking::default_hook::h7de156a9f4432d67
   6:     0x7f664bb87392 - rustc_driver[5180676712772da4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f664b13aad4 - std::panicking::rust_panic_with_hook::h189d11e83e0d3c1d
   8:     0x7f664b13a839 - std::panicking::begin_panic_handler::{{closure}}::h9c14fa00d7507409
   9:     0x7f664b137694 - std::sys_common::backtrace::__rust_end_short_backtrace::hecf0b7131e816200
  10:     0x7f664b13a4e2 - rust_begin_unwind
  11:     0x7f664b0eafc3 - core::panicking::panic_fmt::hffb5048cd1767ea0
  12:     0x7f664b0eb32c - core::panicking::assert_failed_inner::h220e49d01e68d450
  13:     0x7f664bafe7eb - core[9d5edb83b0e1b9c4]::panicking::assert_failed::<usize, usize>
  14:     0x7f664e548ac1 - <rustc_middle[79b39f412ff7aebc]::ty::relate::GeneratorWitness as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  15:     0x7f664e58b7c0 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::binders::<rustc_middle[79b39f412ff7aebc]::ty::relate::GeneratorWitness>
  16:     0x7f664e5373bc - rustc_middle[79b39f412ff7aebc]::ty::relate::super_relate_tys::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  17:     0x7f664e59a863 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::tys
  18:     0x7f664e6335ee - <rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  19:     0x7f664e54bd9c - <&mut rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0} as core[9d5edb83b0e1b9c4]::ops::function::FnOnce<((rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg),)>>::call_once
  20:     0x7f664e57359b - <core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, rustc_middle[79b39f412ff7aebc]::ty::error::TypeError> as rustc_type_ir[c54a98da0e3bcbbf]::InternIteratorElement<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, &rustc_middle[79b39f412ff7aebc]::ty::list::List<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>::intern_with::<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>, <rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt>::mk_substs<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>>::{closure#0}>
  21:     0x7f664e62b08f - <rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt>::mk_substs::<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>>
  22:     0x7f664e478b28 - <rustc_middle[79b39f412ff7aebc]::ty::sty::TraitRef as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  23:     0x7f664e57a414 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::relate::<rustc_middle[79b39f412ff7aebc]::ty::sty::TraitRef>
  24:     0x7f664e64c0b6 - <rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  25:     0x7f664e58b8b6 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::binders::<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate>
  26:     0x7f664e47da26 - <rustc_middle[79b39f412ff7aebc]::ty::sty::Binder<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate> as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  27:     0x7f664e57a236 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::relate::<rustc_middle[79b39f412ff7aebc]::ty::sty::Binder<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate>>
  28:     0x7f664e5face2 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  29:     0x7f664e492c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  30:     0x7f664e6184b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  31:     0x7f664e616001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  32:     0x7f664e615775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  33:     0x7f664e521ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  34:     0x7f664e61b26a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  35:     0x7f664e5fafb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  36:     0x7f664e492c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  37:     0x7f664e6184b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  38:     0x7f664e616001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  39:     0x7f664e615775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  40:     0x7f664e521ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  41:     0x7f664e61b26a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  42:     0x7f664e5fafb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  43:     0x7f664e492c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  44:     0x7f664e6184b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  45:     0x7f664e616001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  46:     0x7f664e615775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  47:     0x7f664e521ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  48:     0x7f664e61b26a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  49:     0x7f664e5fafb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  50:     0x7f664e492c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  51:     0x7f664e6184b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  52:     0x7f664e616001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  53:     0x7f664e615775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  54:     0x7f664e521ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  55:     0x7f664e61b26a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  56:     0x7f664e5fafb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  57:     0x7f664e492c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  58:     0x7f664e6184b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  59:     0x7f664e616001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  60:     0x7f664e615775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  61:     0x7f664e521ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  62:     0x7f664e61b26a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  63:     0x7f664e5fafb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  64:     0x7f664e492c11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  65:     0x7f664e6184b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  66:     0x7f664e616001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  67:     0x7f664e522993 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  68:     0x7f664e5fabcd - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_root_obligation
  69:     0x7f664d1ebedf - rustc_traits[9e65619632aadd08]::evaluate_obligation::evaluate_obligation
  70:     0x7f664d8097b3 - <rustc_query_system[9dff316cef8173b0]::query::config::QueryVTable<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::infer::canonical::Canonical<rustc_middle[79b39f412ff7aebc]::ty::ParamEnvAnd<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>>::compute
  71:     0x7f664da7ac05 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::evaluate_obligation, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
  72:     0x7f664d602058 - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::evaluate_obligation
  73:     0x7f664e5312e6 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt as rustc_trait_selection[53160d62df8e776c]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  74:     0x7f664e53148c - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt as rustc_trait_selection[53160d62df8e776c]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  75:     0x7f664e6438c3 - <rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  76:     0x7f664e641bf8 - <rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillProcessor as rustc_data_structures[27a7519d22327df0]::obligation_forest::ObligationProcessor>::process_obligation
  77:     0x7f664e555d6d - <rustc_data_structures[27a7519d22327df0]::obligation_forest::ObligationForest<rustc_trait_selection[53160d62df8e776c]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillProcessor>
  78:     0x7f664e63ff07 - <rustc_trait_selection[53160d62df8e776c]::traits::fulfill::FulfillmentContext as rustc_infer[d164ecb974ac35ed]::traits::engine::TraitEngine>::select_where_possible
  79:     0x7f664c0b1d96 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_argument_types
  80:     0x7f664c0815a4 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::confirm_builtin_call
  81:     0x7f664c0803fa - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_call
  82:     0x7f664c104c62 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_kind
  83:     0x7f664c097ab1 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  84:     0x7f664c103b42 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  85:     0x7f664c0b9941 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_stmt
  86:     0x7f664c0b9ea7 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_block_with_expected
  87:     0x7f664c104fd8 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_kind
  88:     0x7f664c097ab1 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  89:     0x7f664c103b42 - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  90:     0x7f664c099fbb - <rustc_hir_typeck[97d777f68f886a7c]::fn_ctxt::FnCtxt>::check_return_expr
  91:     0x7f664c292805 - rustc_hir_typeck[97d777f68f886a7c]::check::check_fn
  92:     0x7f664c1c82fb - <rustc_hir_typeck[97d777f68f886a7c]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[97d777f68f886a7c]::typeck_with_fallback<rustc_hir_typeck[97d777f68f886a7c]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[79b39f412ff7aebc]::ty::typeck_results::TypeckResults>
  93:     0x7f664c1e9abf - rustc_hir_typeck[97d777f68f886a7c]::typeck
  94:     0x7f664d9c1243 - rustc_query_system[9dff316cef8173b0]::query::plumbing::try_execute_query::<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_query_system[9dff316cef8173b0]::query::caches::VecCache<rustc_span[fe3b109d6a205fd9]::def_id::LocalDefId, &rustc_middle[79b39f412ff7aebc]::ty::typeck_results::TypeckResults>>
  95:     0x7f664dabc3f6 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::typeck, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
  96:     0x7f664d5ba090 - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::typeck
  97:     0x7f664c15b1d4 - std[ffda4b28d2e2b2d5]::panicking::try::<(), core[9d5edb83b0e1b9c4]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[27a7519d22327df0]::sync::par_for_each_in<&[rustc_span[fe3b109d6a205fd9]::def_id::LocalDefId], <rustc_middle[79b39f412ff7aebc]::hir::map::Map>::par_body_owners<rustc_hir_typeck[97d777f68f886a7c]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  98:     0x7f664c1590ed - rustc_data_structures[27a7519d22327df0]::sync::par_for_each_in::<&[rustc_span[fe3b109d6a205fd9]::def_id::LocalDefId], <rustc_middle[79b39f412ff7aebc]::hir::map::Map>::par_body_owners<rustc_hir_typeck[97d777f68f886a7c]::typeck_item_bodies::{closure#0}>::{closure#0}>
  99:     0x7f664c1e8a97 - rustc_hir_typeck[97d777f68f886a7c]::typeck_item_bodies
 100:     0x7f664d99621e - rustc_query_system[9dff316cef8173b0]::query::plumbing::try_execute_query::<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_query_system[9dff316cef8173b0]::query::caches::DefaultCache<(), ()>>
 101:     0x7f664da79cf4 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::typeck_item_bodies, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
 102:     0x7f664d5b992a - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::typeck_item_bodies
 103:     0x7f664c3c206b - <rustc_session[ab216781b4852cbf]::session::Session>::time::<(), rustc_hir_analysis[51301afa03c42b59]::check_crate::{closure#7}>
 104:     0x7f664c344f23 - rustc_hir_analysis[51301afa03c42b59]::check_crate
 105:     0x7f664bce3921 - rustc_interface[a07a1f0e4c3f1918]::passes::analysis
 106:     0x7f664d986d1b - rustc_query_system[9dff316cef8173b0]::query::plumbing::try_execute_query::<rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_query_system[9dff316cef8173b0]::query::caches::DefaultCache<(), core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>>
 107:     0x7f664dabc7d4 - rustc_query_system[9dff316cef8173b0]::query::plumbing::get_query::<rustc_query_impl[fccb982c32d0468e]::queries::analysis, rustc_query_impl[fccb982c32d0468e]::plumbing::QueryCtxt, rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>
 108:     0x7f664d590fba - <rustc_query_impl[fccb982c32d0468e]::Queries as rustc_middle[79b39f412ff7aebc]::ty::query::QueryEngine>::analysis
 109:     0x7f664bbe4e1c - <rustc_interface[a07a1f0e4c3f1918]::passes::QueryContext>::enter::<rustc_driver[5180676712772da4]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 110:     0x7f664bbee9ba - <rustc_interface[a07a1f0e4c3f1918]::interface::Compiler>::enter::<rustc_driver[5180676712772da4]::run_compiler::{closure#1}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<core[9d5edb83b0e1b9c4]::option::Option<rustc_interface[a07a1f0e4c3f1918]::queries::Linker>, rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 111:     0x7f664bb88afc - rustc_span[fe3b109d6a205fd9]::with_source_map::<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
 112:     0x7f664bbe3cfa - <scoped_tls[3671739d64d07b67]::ScopedKey<rustc_span[fe3b109d6a205fd9]::SessionGlobals>>::set::<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 113:     0x7f664bba6caf - std[ffda4b28d2e2b2d5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a07a1f0e4c3f1918]::util::run_in_thread_pool_with_globals<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 114:     0x7f664bbe9f76 - std[ffda4b28d2e2b2d5]::panic::catch_unwind::<core[9d5edb83b0e1b9c4]::panic::unwind_safe::AssertUnwindSafe<<std[ffda4b28d2e2b2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[a07a1f0e4c3f1918]::util::run_in_thread_pool_with_globals<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>
 115:     0x7f664bb991b5 - <<std[ffda4b28d2e2b2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[a07a1f0e4c3f1918]::util::run_in_thread_pool_with_globals<rustc_interface[a07a1f0e4c3f1918]::interface::run_compiler<core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>, rustc_driver[5180676712772da4]::run_compiler::{closure#1}>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<(), rustc_errors[7c57b54439ff54a3]::ErrorGuaranteed>>::{closure#1} as core[9d5edb83b0e1b9c4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 116:     0x7f664b1478ce - std::sys::unix::thread::Thread::new::thread_start::hd5e033cd96f77265
 117:     0x7f664aedcb43 - <unknown>
 118:     0x7f664af6ea00 - <unknown>
 119:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (d83a85bc3 2022-12-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Send`
#1 [typeck] type-checking `pass_assert`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/async-await/issue-105501.rs stdout ----
---- [ui] src/test/ui/async-await/issue-105501.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-105501.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-105501" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-105501/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  left: `0`,
 right: `5`', /checkout/compiler/rustc_middle/src/ty/relate.rs:355:9
stack backtrace:
stack backtrace:
   0:     0x7f535214f175 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3654c56af00a5f5e
   1:     0x7f53521bf1e8 - core::fmt::write::h2094bcbce382bdd5
   2:     0x7f5352141291 - std::io::Write::write_fmt::h8656690cbcb421cc
   3:     0x7f535214ef81 - std::sys_common::backtrace::print::hb2648e0ef907ed83
   4:     0x7f5352152364 - std::panicking::default_hook::{{closure}}::h57dad3e290d67faf
   5:     0x7f535215202a - std::panicking::default_hook::h7de156a9f4432d67
   6:     0x7f5352b9f392 - rustc_driver[5180676712772da4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5352152ad4 - std::panicking::rust_panic_with_hook::h189d11e83e0d3c1d
   8:     0x7f5352152839 - std::panicking::begin_panic_handler::{{closure}}::h9c14fa00d7507409
   9:     0x7f535214f694 - std::sys_common::backtrace::__rust_end_short_backtrace::hecf0b7131e816200
  10:     0x7f53521524e2 - rust_begin_unwind
  11:     0x7f5352102fc3 - core::panicking::panic_fmt::hffb5048cd1767ea0
  12:     0x7f535210332c - core::panicking::assert_failed_inner::h220e49d01e68d450
  13:     0x7f5352b167eb - core[9d5edb83b0e1b9c4]::panicking::assert_failed::<usize, usize>
  14:     0x7f5355560ac1 - <rustc_middle[79b39f412ff7aebc]::ty::relate::GeneratorWitness as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  15:     0x7f53555a37c0 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::binders::<rustc_middle[79b39f412ff7aebc]::ty::relate::GeneratorWitness>
  16:     0x7f535554f3bc - rustc_middle[79b39f412ff7aebc]::ty::relate::super_relate_tys::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  17:     0x7f53555b2863 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::tys
  18:     0x7f535564b5ee - <rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  19:     0x7f5355563d9c - <&mut rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0} as core[9d5edb83b0e1b9c4]::ops::function::FnOnce<((rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg),)>>::call_once
  20:     0x7f535558b59b - <core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, rustc_middle[79b39f412ff7aebc]::ty::error::TypeError> as rustc_type_ir[c54a98da0e3bcbbf]::InternIteratorElement<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg, &rustc_middle[79b39f412ff7aebc]::ty::list::List<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>::intern_with::<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>, <rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt>::mk_substs<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>>::{closure#0}>
  21:     0x7f535564308f - <rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt>::mk_substs::<core[9d5edb83b0e1b9c4]::iter::adapters::map::Map<core[9d5edb83b0e1b9c4]::iter::adapters::zip::Zip<core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>, core[9d5edb83b0e1b9c4]::iter::adapters::copied::Copied<core[9d5edb83b0e1b9c4]::slice::iter::Iter<rustc_middle[79b39f412ff7aebc]::ty::subst::GenericArg>>>, rustc_middle[79b39f412ff7aebc]::ty::relate::relate_substs<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>::{closure#0}>>
  22:     0x7f5355490b28 - <rustc_middle[79b39f412ff7aebc]::ty::sty::TraitRef as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  23:     0x7f5355592414 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::relate::<rustc_middle[79b39f412ff7aebc]::ty::sty::TraitRef>
  24:     0x7f53556640b6 - <rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  25:     0x7f53555a38b6 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::binders::<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate>
  26:     0x7f5355495a26 - <rustc_middle[79b39f412ff7aebc]::ty::sty::Binder<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate> as rustc_middle[79b39f412ff7aebc]::ty::relate::Relate>::relate::<rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer>
  27:     0x7f5355592236 - <rustc_trait_selection[53160d62df8e776c]::traits::select::only_fresh_differ::FreshDiffer as rustc_middle[79b39f412ff7aebc]::ty::relate::TypeRelation>::relate::<rustc_middle[79b39f412ff7aebc]::ty::sty::Binder<rustc_middle[79b39f412ff7aebc]::ty::TraitPredicate>>
  28:     0x7f5355612ce2 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  29:     0x7f53554aac11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  30:     0x7f53556304b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  31:     0x7f535562e001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  32:     0x7f535562d775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  33:     0x7f5355539ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  34:     0x7f535563326a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  35:     0x7f5355612fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  36:     0x7f53554aac11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  37:     0x7f53556304b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  38:     0x7f535562e001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  39:     0x7f535562d775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  40:     0x7f5355539ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  41:     0x7f535563326a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  42:     0x7f5355612fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  43:     0x7f53554aac11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  44:     0x7f53556304b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  45:     0x7f535562e001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  46:     0x7f535562d775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  47:     0x7f5355539ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  48:     0x7f535563326a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  49:     0x7f5355612fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  50:     0x7f53554aac11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  51:     0x7f53556304b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  52:     0x7f535562e001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  53:     0x7f535562d775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  54:     0x7f5355539ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  55:     0x7f535563326a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  56:     0x7f5355612fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  57:     0x7f53554aac11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  58:     0x7f53556304b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  59:     0x7f535562e001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
  60:     0x7f535562d775 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[51e3d04528e252cd]::vec::into_iter::IntoIter<rustc_infer[d164ecb974ac35ed]::traits::Obligation<rustc_middle[79b39f412ff7aebc]::ty::Predicate>>>
  61:     0x7f5355539ed0 - <rustc_infer[d164ecb974ac35ed]::infer::InferCtxt>::probe::<core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}::{closure#0}>::{closure#0}>
  62:     0x7f535563326a - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_candidate
  63:     0x7f5355612fb8 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_stack
  64:     0x7f53554aac11 - <rustc_query_system[9dff316cef8173b0]::dep_graph::graph::DepGraph<rustc_middle[79b39f412ff7aebc]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[79b39f412ff7aebc]::ty::context::TyCtxt, <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>::{closure#0}, core[9d5edb83b0e1b9c4]::result::Result<rustc_middle[79b39f412ff7aebc]::traits::select::EvaluationResult, rustc_middle[79b39f412ff7aebc]::traits::select::OverflowError>>
  65:     0x7f53556304b5 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  66:     0x7f535562e001 - <rustc_trait_selection[53160d62df8e776c]::traits::select::SelectionContext>::evaluate_predicate_recursively
---
To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
   |
   |
LL | fn cycle1() -> impl Clone {
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `cycle1` for borrow checking...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL |     send(cycle2().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
note: ...which requires computing type of `cycle2::{opaque#0}`...
   |
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `cycle2` for borrow checking...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL |     send(cycle1().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
   = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | | fn send<T: Send>(_: T) {}
...  |
LL | |     Rc::new(String::from("foo"))
LL | | }


error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
   |
   |
LL | fn cycle1() -> impl Clone {
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `cycle1` for borrow checking...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL |     send(cycle2().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
note: ...which requires computing type of `cycle2::{opaque#0}`...
   |
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `cycle2` for borrow checking...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | | fn send<T: Send>(_: T) {}
...  |
LL | |     Rc::new(String::from("foo"))
LL | | }


error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
   |
   |
LL | fn cycle1() -> impl Clone {
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `cycle1` for borrow checking...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL |     send(cycle2().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
note: ...which requires computing type of `cycle2::{opaque#0}`...
   |
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `cycle2` for borrow checking...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | | fn send<T: Send>(_: T) {}
...  |
LL | |     Rc::new(String::from("foo"))
LL | | }

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/auto-trait-leakage3.rs stdout ----
diff of stderr:

17 LL | mod m {
19 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
+    |
+ LL |     type Foo = impl std::fmt::Debug;
+    |                ^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: ...which requires type-checking `m::bar`...
+    |
+ LL |     pub fn bar() {
+    |     ^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^
+    = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
+ note: cycle used when checking item types in module `m`
+    |
+    |
+ LL | mod m {
+ 
+ 
+ error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
+    |
+ LL |     type Foo = impl std::fmt::Debug;
+    |                ^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: ...which requires type-checking `m::bar`...
+    |
+ LL |     pub fn bar() {
+    |     ^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^
+    = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
+ note: cycle used when checking item types in module `m`
+    |
+    |
+ LL | mod m {
+ 
+ error: aborting due to 3 previous errors
21 
22 For more information about this error, try `rustc --explain E0391`.
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/auto-trait-leakage3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
   |
LL |         is_send(foo());
   |         ^^^^^^^
   = note: ...which requires evaluating trait selection obligation `m::Foo: core::marker::Send`...
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in module `m`
   |
   |
LL | mod m {


error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
LL |     pub fn bar() {
   |     ^^^^^^^^^^^^
   |     ^^^^^^^^^^^^
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in module `m`
   |
   |
LL | mod m {


error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
LL |     pub fn bar() {
   |     ^^^^^^^^^^^^
   |     ^^^^^^^^^^^^
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in module `m`
   |
   |
LL | mod m {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/inference-cycle.rs stdout ----
diff of stderr:

17 LL | mod m {
19 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
+    |
+ LL |     type Foo = impl std::fmt::Debug;
+    |                ^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: ...which requires type-checking `m::bar`...
+    |
+ LL |     pub fn bar() {
+    |     ^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^
+    = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
+ note: cycle used when checking item types in module `m`
+    |
+    |
+ LL | mod m {
+ 
+ 
+ error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
+    |
+ LL |     type Foo = impl std::fmt::Debug;
+    |                ^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: ...which requires type-checking `m::bar`...
+    |
+ LL |     pub fn bar() {
+    |     ^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^
+    = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
+ note: cycle used when checking item types in module `m`
+    |
+    |
+ LL | mod m {
+ 
+ error: aborting due to 3 previous errors
21 
22 For more information about this error, try `rustc --explain E0391`.
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/inference-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/inference-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/inference-cycle" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/inference-cycle/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
   |
LL |         is_send(foo()); // Today: error
   |         ^^^^^^^
   = note: ...which requires evaluating trait selection obligation `m::Foo: core::marker::Send`...
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in module `m`
   |
   |
LL | mod m {


error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
LL |     pub fn bar() {
   |     ^^^^^^^^^^^^
   |     ^^^^^^^^^^^^
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in module `m`
   |
   |
LL | mod m {


error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
LL |     pub fn bar() {
   |     ^^^^^^^^^^^^
   |     ^^^^^^^^^^^^
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in module `m`
   |
   |
LL | mod m {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0391`.
---
23 LL | | fn main() {}
24    | |____________^
25 
- error: aborting due to previous error
+ error[E0391]: cycle detected when computing type of `Foo::{opaque#0}`
+   --> $DIR/reveal_local.rs:5:12
+ LL | type Foo = impl Debug;
+    |            ^^^^^^^^^^
+    |
+    |
+ note: ...which requires type-checking `not_good`...
+   --> $DIR/reveal_local.rs:10:1
+    |
+ LL | fn not_good() {
+    | ^^^^^^^^^^^^^
+    = note: ...which again requires computing type of `Foo::{opaque#0}`, completing the cycle
+ note: cycle used when checking item types in top-level module
+   --> $DIR/reveal_local.rs:1:1
+    |
+ LL | / #![feature(type_alias_impl_trait)]
+ LL | |
+ LL | | use std::fmt::Debug;
+ LL | |
+ LL | |
+ LL | | fn main() {}
+    | |____________^
+ 
+ 
+ error[E0391]: cycle detected when computing type of `Foo::{opaque#0}`
+   --> $DIR/reveal_local.rs:5:12
+ LL | type Foo = impl Debug;
+    |            ^^^^^^^^^^
+    |
+    |
+ note: ...which requires type-checking `not_gooder`...
+   --> $DIR/reveal_local.rs:16:1
+    |
+ LL | fn not_gooder() {
+    | ^^^^^^^^^^^^^^^
+    = note: ...which again requires computing type of `Foo::{opaque#0}`, completing the cycle
+ note: cycle used when checking item types in top-level module
+   --> $DIR/reveal_local.rs:1:1
+    |
+ LL | / #![feature(type_alias_impl_trait)]
+ LL | |
+ LL | | use std::fmt::Debug;
+ LL | |
+ LL | |
+ LL | | fn main() {}
+    | |____________^
+ 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/reveal_local.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/reveal_local.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/reveal_local" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/reveal_local/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Foo::{opaque#0}`
   |
LL | type Foo = impl Debug;
   |            ^^^^^^^^^^
   |
   |
note: ...which requires type-checking `not_good`...
   |
LL |     is_send::<Foo>();
   |     ^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `Foo: core::marker::Send`...
   = note: ...which again requires computing type of `Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / #![feature(type_alias_impl_trait)]
LL | |
LL | | use std::fmt::Debug;
LL | | use std::fmt::Debug;
LL | |
...  |
LL | |
LL | | fn main() {}
   | |____________^

error[E0391]: cycle detected when computing type of `Foo::{opaque#0}`
   |
LL | type Foo = impl Debug;
   |            ^^^^^^^^^^
   |
   |
note: ...which requires type-checking `not_good`...
   |
   |
LL | fn not_good() {
   | ^^^^^^^^^^^^^
   = note: ...which again requires computing type of `Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / #![feature(type_alias_impl_trait)]
LL | |
LL | | use std::fmt::Debug;
LL | | use std::fmt::Debug;
LL | |
...  |
LL | |
LL | | fn main() {}
   | |____________^

error[E0391]: cycle detected when computing type of `Foo::{opaque#0}`
   |
LL | type Foo = impl Debug;
   |            ^^^^^^^^^^
   |
   |
note: ...which requires type-checking `not_gooder`...
   |
   |
LL | fn not_gooder() {
   | ^^^^^^^^^^^^^^^
   = note: ...which again requires computing type of `Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / #![feature(type_alias_impl_trait)]
LL | |
LL | | use std::fmt::Debug;
