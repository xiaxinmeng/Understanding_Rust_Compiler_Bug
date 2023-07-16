plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v1.0.0
    Checking adler v1.0.2
    Checking rustc-demangle v0.1.21
error[E0412]: cannot find type `DEFAULT_COOP_PREFERRED` in this scope
     |
     |
1610 | impl<T> From<BinaryHeap<T>> for Vec<T, Global, DEFAULT_COOP_PREFERRED> {
     |       |
     |       |
     |       help: you might be missing a type parameter: `, DEFAULT_COOP_PREFERRED`
error[E0412]: cannot find type `Global` in this scope
   --> library/alloc/src/ffi/c_str.rs:726:32
    |
    |
726 | impl From<CString> for Vec<u8, Global, DEFAULT_COOP_PREFERRED> {
    |
help: consider importing this struct
    |
4   | use crate::Global;
4   | use crate::Global;
    |

error[E0412]: cannot find type `DEFAULT_COOP_PREFERRED` in this scope
    |
    |
726 | impl From<CString> for Vec<u8, Global, DEFAULT_COOP_PREFERRED> {
    |     |
    |     |
    |     help: you might be missing a type parameter: `<DEFAULT_COOP_PREFERRED>`
error[E0412]: cannot find type `Global` in this scope
   --> library/alloc/src/vec/in_place_collect.rs:153:42
    |
    |
153 | impl<T, I> SpecFromIter<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
    |
help: consider importing this struct
    |
140 | use crate::Global;
140 | use crate::Global;
    |

error[E0412]: cannot find type `DEFAULT_COOP_PREFERRED` in this scope
    |
    |
153 | impl<T, I> SpecFromIter<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
    |          |
    |          |
    |          help: you might be missing a type parameter: `, DEFAULT_COOP_PREFERRED`
error[E0412]: cannot find type `Global` in this scope
  --> library/alloc/src/vec/spec_from_iter_nested.rs:16:48
   |
   |
16 | impl<T, I> SpecFromIterNested<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
   |
help: consider importing this struct
   |
1  | use crate::Global;
1  | use crate::Global;
   |

error[E0412]: cannot find type `DEFAULT_COOP_PREFERRED` in this scope
   |
   |
16 | impl<T, I> SpecFromIterNested<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
   |          |
   |          |
   |          help: you might be missing a type parameter: `, DEFAULT_COOP_PREFERRED`
error[E0412]: cannot find type `Global` in this scope
  --> library/alloc/src/vec/spec_from_iter_nested.rs:48:48
   |
   |
48 | impl<T, I> SpecFromIterNested<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
   |
help: consider importing this struct
   |
1  | use crate::Global;
1  | use crate::Global;
   |

error[E0412]: cannot find type `DEFAULT_COOP_PREFERRED` in this scope
   |
   |
48 | impl<T, I> SpecFromIterNested<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
   |          |
   |          |
   |          help: you might be missing a type parameter: `, DEFAULT_COOP_PREFERRED`
error[E0412]: cannot find type `Global` in this scope
  --> library/alloc/src/vec/spec_from_iter.rs:28:42
   |
   |
28 | impl<T, I> SpecFromIter<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
   |
help: consider importing this struct
   |
1  | use crate::Global;
1  | use crate::Global;
   |

error[E0412]: cannot find type `DEFAULT_COOP_PREFERRED` in this scope
   |
   |
28 | impl<T, I> SpecFromIter<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
   |          |
   |          |
   |          help: you might be missing a type parameter: `, DEFAULT_COOP_PREFERRED`
error[E0412]: cannot find type `Global` in this scope
  --> library/alloc/src/vec/spec_from_iter.rs:37:50
   |
   |
37 | impl<T> SpecFromIter<T, IntoIter<T>> for Vec<T,  Global, DEFAULT_COOP_PREFERRED> {
   |
help: consider importing this struct
   |
1  | use crate::Global;
1  | use crate::Global;
   |

error[E0412]: cannot find type `DEFAULT_COOP_PREFERRED` in this scope
   |
   |
37 | impl<T> SpecFromIter<T, IntoIter<T>> for Vec<T,  Global, DEFAULT_COOP_PREFERRED> {
   |       |
   |       |
   |       help: you might be missing a type parameter: `, DEFAULT_COOP_PREFERRED`
error[E0747]: unresolved item provided when a constant was expected
    --> library/alloc/src/collections/binary_heap.rs:1610:48
     |
     |
1610 | impl<T> From<BinaryHeap<T>> for Vec<T, Global, DEFAULT_COOP_PREFERRED> {
     |
     |
help: if this generic argument was intended as a const parameter, surround it with braces
     |
1610 | impl<T> From<BinaryHeap<T>> for Vec<T, Global, { DEFAULT_COOP_PREFERRED }> {

error[E0747]: unresolved item provided when a constant was expected
   --> library/alloc/src/ffi/c_str.rs:726:40
    |
    |
726 | impl From<CString> for Vec<u8, Global, DEFAULT_COOP_PREFERRED> {
    |
    |
help: if this generic argument was intended as a const parameter, surround it with braces
    |
726 | impl From<CString> for Vec<u8, Global, { DEFAULT_COOP_PREFERRED }> {

error[E0747]: unresolved item provided when a constant was expected
   --> library/alloc/src/vec/in_place_collect.rs:153:50
    |
    |
153 | impl<T, I> SpecFromIter<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
    |
    |
help: if this generic argument was intended as a const parameter, surround it with braces
    |
153 | impl<T, I> SpecFromIter<T, I> for Vec<T, Global, { DEFAULT_COOP_PREFERRED }>

error[E0747]: unresolved item provided when a constant was expected
  --> library/alloc/src/vec/spec_from_iter_nested.rs:16:56
   |
   |
16 | impl<T, I> SpecFromIterNested<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
16 | impl<T, I> SpecFromIterNested<T, I> for Vec<T, Global, { DEFAULT_COOP_PREFERRED }>

error[E0747]: unresolved item provided when a constant was expected
  --> library/alloc/src/vec/spec_from_iter_nested.rs:48:56
   |
   |
48 | impl<T, I> SpecFromIterNested<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
48 | impl<T, I> SpecFromIterNested<T, I> for Vec<T, Global, { DEFAULT_COOP_PREFERRED }>

error[E0747]: unresolved item provided when a constant was expected
  --> library/alloc/src/vec/spec_from_iter.rs:28:50
   |
   |
28 | impl<T, I> SpecFromIter<T, I> for Vec<T, Global, DEFAULT_COOP_PREFERRED>
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
28 | impl<T, I> SpecFromIter<T, I> for Vec<T, Global, { DEFAULT_COOP_PREFERRED }>

error[E0747]: unresolved item provided when a constant was expected
  --> library/alloc/src/vec/spec_from_iter.rs:37:58
   |
   |
37 | impl<T> SpecFromIter<T, IntoIter<T>> for Vec<T,  Global, DEFAULT_COOP_PREFERRED> {
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
37 | impl<T> SpecFromIter<T, IntoIter<T>> for Vec<T,  Global, { DEFAULT_COOP_PREFERRED }> {


error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:869:9: const parameter `COOP_PREFERRED/#2` (Const { ty: bool, kind: Param(COOP_PREFERRED/#2) }/2) out of range when substituting substs=[_, alloc::Global]
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/compiler/rustc_errors/src/lib.rs:1576:9
stack backtrace:
stack backtrace:
   0:     0x7fefe8f856fa - std::backtrace_rs::backtrace::libunwind::trace::h89c8352810b61eca
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fefe8f856fa - std::backtrace_rs::backtrace::trace_unsynchronized::h0bdd452633778353
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fefe8f856fa - std::sys_common::backtrace::_print_fmt::hefd3479133f53c8c
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fefe8f856fa - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed2134e39e4b6848
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fefe8fe825e - core::fmt::write::h88625ee2c64a5420
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/core/src/fmt/mod.rs:1208:17
   5:     0x7fefe8f75b85 - std::io::Write::write_fmt::he11e7624951301d9
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/io/mod.rs:1682:15
   6:     0x7fefe8f854c5 - std::sys_common::backtrace::_print::h7f2832a74872d3db
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fefe8f854c5 - std::sys_common::backtrace::print::h0f2accac11ca9894
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fefe8f8820f - std::panicking::default_hook::{{closure}}::haee8e7d2bcd0c5c8
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/panicking.rs:267:22
   9:     0x7fefe8f87f4b - std::panicking::default_hook::hdbef06314f945eae
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/panicking.rs:286:9
  10:     0x7fefe7d7a4e1 - rustc_driver[722cf9eaf3bfc82d]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fefe8f88a4d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hb727f4a90fb5bebb
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/alloc/src/boxed.rs:2032:9
  12:     0x7fefe8f88a4d - std::panicking::rust_panic_with_hook::h5380fb487f7ca9ce
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/panicking.rs:692:13
  13:     0x7fefe8205c21 - std[e6bb1224541cc9dc]::panicking::begin_panic::<rustc_errors[b3a8aea407dcd03e]::ExplicitBug>::{closure#0}
  14:     0x7fefe82056a6 - std[e6bb1224541cc9dc]::sys_common::backtrace::__rust_end_short_backtrace::<std[e6bb1224541cc9dc]::panicking::begin_panic<rustc_errors[b3a8aea407dcd03e]::ExplicitBug>::{closure#0}, !>
  15:     0x7fefe823f176 - std[e6bb1224541cc9dc]::panicking::begin_panic::<rustc_errors[b3a8aea407dcd03e]::ExplicitBug>
  16:     0x7fefe82054e6 - std[e6bb1224541cc9dc]::panic::panic_any::<rustc_errors[b3a8aea407dcd03e]::ExplicitBug>
  17:     0x7fefe8201c96 - <rustc_errors[b3a8aea407dcd03e]::HandlerInner>::bug::<&alloc[bebb2c59904f30a5]::string::String>
  18:     0x7fefe8201900 - <rustc_errors[b3a8aea407dcd03e]::Handler>::bug::<&alloc[bebb2c59904f30a5]::string::String>
  19:     0x7fefe81e394e - rustc_middle[c3e0e3d1153b9c4a]::ty::context::tls::with_context_opt::<rustc_middle[c3e0e3d1153b9c4a]::ty::context::tls::with_opt<rustc_middle[c3e0e3d1153b9c4a]::util::bug::opt_span_bug_fmt<rustc_span[c84e21eab4fde2cd]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7fefe81e4db6 - rustc_middle[c3e0e3d1153b9c4a]::util::bug::opt_span_bug_fmt::<rustc_span[c84e21eab4fde2cd]::span_encoding::Span>
  21:     0x7fefe5ff7903 - rustc_middle[c3e0e3d1153b9c4a]::util::bug::bug_fmt
  22:     0x7fefe8236e3e - <rustc_middle[c3e0e3d1153b9c4a]::ty::subst::SubstFolder>::const_param_out_of_range
  23:     0x7fefe5f193af - <rustc_middle[c3e0e3d1153b9c4a]::ty::subst::SubstFolder as rustc_middle[c3e0e3d1153b9c4a]::ty::fold::FallibleTypeFolder>::try_fold_const
  24:     0x7fefe5f1a303 - <&rustc_middle[c3e0e3d1153b9c4a]::ty::list::List<rustc_middle[c3e0e3d1153b9c4a]::ty::subst::GenericArg> as rustc_middle[c3e0e3d1153b9c4a]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[c3e0e3d1153b9c4a]::ty::subst::SubstFolder>
  25:     0x7fefe5f1d110 - <rustc_middle[c3e0e3d1153b9c4a]::ty::consts::Const as rustc_middle[c3e0e3d1153b9c4a]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle[c3e0e3d1153b9c4a]::ty::subst::SubstFolder>
  26:     0x7fefe5d00401 - <rustc_middle[c3e0e3d1153b9c4a]::ty::subst::SubstFolder as rustc_middle[c3e0e3d1153b9c4a]::ty::fold::FallibleTypeFolder>::try_fold_ty
  27:     0x7fefe700a86f - <rustc_middle[c3e0e3d1153b9c4a]::ty::subst::GenericArg as rustc_middle[c3e0e3d1153b9c4a]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[c3e0e3d1153b9c4a]::ty::subst::SubstFolder>
  28:     0x7fefe5d74f45 - <rustc_middle[c3e0e3d1153b9c4a]::ty::generics::GenericPredicates>::instantiate_into
  29:     0x7fefe63c5243 - <rustc_trait_selection[9b9800b1772785a4]::traits::wf::WfPredicates>::nominal_obligations_inner
  30:     0x7fefe63c200d - <rustc_trait_selection[9b9800b1772785a4]::traits::wf::WfPredicates>::compute
  31:     0x7fefe63bcf11 - rustc_trait_selection[9b9800b1772785a4]::traits::wf::obligations
  32:     0x7fefe665c7a0 - <rustc_trait_selection[9b9800b1772785a4]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[bebb2c59904f30a5]::vec::into_iter::IntoIter<rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>>>
  33:     0x7fefe6006d2f - <rustc_trait_selection[9b9800b1772785a4]::traits::select::SelectionContext>::evaluate_candidate
  34:     0x7fefe5fd4520 - <rustc_trait_selection[9b9800b1772785a4]::traits::select::SelectionContext>::select
  35:     0x7fefe643dfd0 - rustc_trait_selection[9b9800b1772785a4]::traits::project::opt_normalize_projection_type
  36:     0x7fefe64a202c - <rustc_infer[87152b7f9c12d6e]::infer::InferCtxt>::commit_if_ok::<rustc_trait_selection[9b9800b1772785a4]::traits::project::ProjectAndUnifyResult, rustc_infer[87152b7f9c12d6e]::traits::project::MismatchedProjectionTypes, rustc_trait_selection[9b9800b1772785a4]::traits::project::poly_project_and_unify_type::{closure#0}>
  37:     0x7fefe67d6cde - <rustc_trait_selection[9b9800b1772785a4]::traits::select::SelectionContext>::evaluate_predicate_recursively
  38:     0x7fefe71d4b43 - <rustc_infer[87152b7f9c12d6e]::infer::InferCtxt>::probe::<core[1bdb63ea3d2475fb]::result::Result<rustc_middle[c3e0e3d1153b9c4a]::traits::select::EvaluationResult, rustc_middle[c3e0e3d1153b9c4a]::traits::select::OverflowError>, <rustc_trait_selection[9b9800b1772785a4]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[9b9800b1772785a4]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  39:     0x7fefe70b6e34 - <core[1bdb63ea3d2475fb]::iter::adapters::copied::Copied<core[1bdb63ea3d2475fb]::slice::iter::Iter<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>> as core[1bdb63ea3d2475fb]::iter::traits::iterator::Iterator>::try_fold::<(), &mut core[1bdb63ea3d2475fb]::iter::adapters::map::map_try_fold<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate, rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate, (), core[1bdb63ea3d2475fb]::ops::control_flow::ControlFlow<rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>>, rustc_trait_selection[9b9800b1772785a4]::traits::coherence::implicit_negative::{closure#0}, core[1bdb63ea3d2475fb]::iter::adapters::map::map_try_fold<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate, rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>, (), core[1bdb63ea3d2475fb]::ops::control_flow::ControlFlow<rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>>, rustc_trait_selection[9b9800b1772785a4]::traits::coherence::implicit_negative::{closure#1}, core[1bdb63ea3d2475fb]::iter::traits::iterator::Iterator::find::check<rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>, &mut rustc_trait_selection[9b9800b1772785a4]::traits::coherence::implicit_negative::{closure#2}>::{closure#0}>::{closure#0}>::{closure#0}, core[1bdb63ea3d2475fb]::ops::control_flow::ControlFlow<rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>>>
  40:     0x7fefe70b6bb2 - <core[1bdb63ea3d2475fb]::iter::adapters::chain::Chain<core[1bdb63ea3d2475fb]::iter::adapters::copied::Copied<core[1bdb63ea3d2475fb]::slice::iter::Iter<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>>, alloc[bebb2c59904f30a5]::vec::into_iter::IntoIter<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>> as core[1bdb63ea3d2475fb]::iter::traits::iterator::Iterator>::try_fold::<(), core[1bdb63ea3d2475fb]::iter::adapters::map::map_try_fold<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate, rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate, (), core[1bdb63ea3d2475fb]::ops::control_flow::ControlFlow<rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>>, rustc_trait_selection[9b9800b1772785a4]::traits::coherence::implicit_negative::{closure#0}, core[1bdb63ea3d2475fb]::iter::adapters::map::map_try_fold<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate, rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>, (), core[1bdb63ea3d2475fb]::ops::control_flow::ControlFlow<rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>>, rustc_trait_selection[9b9800b1772785a4]::traits::coherence::implicit_negative::{closure#1}, core[1bdb63ea3d2475fb]::iter::traits::iterator::Iterator::find::check<rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>, &mut rustc_trait_selection[9b9800b1772785a4]::traits::coherence::implicit_negative::{closure#2}>::{closure#0}>::{closure#0}>::{closure#0}, core[1bdb63ea3d2475fb]::ops::control_flow::ControlFlow<rustc_infer[87152b7f9c12d6e]::traits::Obligation<rustc_middle[c3e0e3d1153b9c4a]::ty::Predicate>>>
  41:     0x7fefe70b46e0 - rustc_trait_selection[9b9800b1772785a4]::traits::coherence::overlap_within_probe
  42:     0x7fefe70b3baa - <rustc_infer[87152b7f9c12d6e]::infer::InferCtxt>::probe_maybe_skip_leak_check::<core[1bdb63ea3d2475fb]::option::Option<rustc_trait_selection[9b9800b1772785a4]::traits::coherence::OverlapResult>, rustc_trait_selection[9b9800b1772785a4]::traits::coherence::overlap::{closure#0}>
  43:     0x7fefe70b0e6a - <rustc_middle[c3e0e3d1153b9c4a]::traits::specialization_graph::Children as rustc_trait_selection[9b9800b1772785a4]::traits::specialize::specialization_graph::ChildrenExt>::insert
  44:     0x7fefe70adccd - <rustc_middle[c3e0e3d1153b9c4a]::traits::specialization_graph::Graph as rustc_trait_selection[9b9800b1772785a4]::traits::specialize::specialization_graph::GraphExt>::insert
  45:     0x7fefe70acc81 - rustc_trait_selection[9b9800b1772785a4]::traits::specialize::specialization_graph_provider
  46:     0x7fefe6955b4a - rustc_query_system[496c52d9687661d8]::query::plumbing::try_execute_query::<rustc_query_impl[67838e5c34812a10]::plumbing::QueryCtxt, rustc_query_system[496c52d9687661d8]::query::caches::ArenaCache<rustc_span[c84e21eab4fde2cd]::def_id::DefId, rustc_middle[c3e0e3d1153b9c4a]::traits::specialization_graph::Graph>>
  47:     0x7fefe70088db - rustc_query_system[496c52d9687661d8]::query::plumbing::get_query::<rustc_query_impl[67838e5c34812a10]::queries::specialization_graph_of, rustc_query_impl[67838e5c34812a10]::plumbing::QueryCtxt>
  48:     0x7fefe7153795 - rustc_hir_analysis[4a5314653546508e]::coherence::coherent_trait
  49:     0x7fefe64b4868 - rustc_query_system[496c52d9687661d8]::query::plumbing::try_execute_query::<rustc_query_impl[67838e5c34812a10]::plumbing::QueryCtxt, rustc_query_system[496c52d9687661d8]::query::caches::DefaultCache<rustc_span[c84e21eab4fde2cd]::def_id::DefId, ()>>
  50:     0x7fefe70dce82 - rustc_query_system[496c52d9687661d8]::query::plumbing::get_query::<rustc_query_impl[67838e5c34812a10]::queries::coherent_trait, rustc_query_impl[67838e5c34812a10]::plumbing::QueryCtxt>
  51:     0x7fefe62719d2 - <rustc_session[63e09809bc07dcca]::session::Session>::track_errors::<rustc_hir_analysis[4a5314653546508e]::check_crate::{closure#3}, ()>
  52:     0x7fefe626e93c - rustc_hir_analysis[4a5314653546508e]::check_crate
  53:     0x7fefe626e60b - rustc_interface[4c7fb0143921c886]::passes::analysis
  54:     0x7fefe752ac1a - rustc_query_system[496c52d9687661d8]::query::plumbing::try_execute_query::<rustc_query_impl[67838e5c34812a10]::plumbing::QueryCtxt, rustc_query_system[496c52d9687661d8]::query::caches::DefaultCache<(), core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>>>
  55:     0x7fefe752a910 - rustc_query_system[496c52d9687661d8]::query::plumbing::get_query::<rustc_query_impl[67838e5c34812a10]::queries::analysis, rustc_query_impl[67838e5c34812a10]::plumbing::QueryCtxt>
  56:     0x7fefe6f25b83 - <rustc_interface[4c7fb0143921c886]::passes::QueryContext>::enter::<rustc_driver[722cf9eaf3bfc82d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>>
  57:     0x7fefe6f220c3 - <rustc_interface[4c7fb0143921c886]::interface::Compiler>::enter::<rustc_driver[722cf9eaf3bfc82d]::run_compiler::{closure#1}::{closure#2}, core[1bdb63ea3d2475fb]::result::Result<core[1bdb63ea3d2475fb]::option::Option<rustc_interface[4c7fb0143921c886]::queries::Linker>, rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>>
  58:     0x7fefe6f1d118 - rustc_span[c84e21eab4fde2cd]::with_source_map::<core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>, rustc_interface[4c7fb0143921c886]::interface::run_compiler<core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>, rustc_driver[722cf9eaf3bfc82d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  59:     0x7fefe6f1cc05 - <scoped_tls[b3215e5d4e75a866]::ScopedKey<rustc_span[c84e21eab4fde2cd]::SessionGlobals>>::set::<rustc_interface[4c7fb0143921c886]::interface::run_compiler<core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>, rustc_driver[722cf9eaf3bfc82d]::run_compiler::{closure#1}>::{closure#0}, core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>>
  60:     0x7fefe6f1c1f2 - std[e6bb1224541cc9dc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4c7fb0143921c886]::util::run_in_thread_pool_with_globals<rustc_interface[4c7fb0143921c886]::interface::run_compiler<core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>, rustc_driver[722cf9eaf3bfc82d]::run_compiler::{closure#1}>::{closure#0}, core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>>
  61:     0x7fefe762b78a - <<std[e6bb1224541cc9dc]::thread::Builder>::spawn_unchecked_<rustc_interface[4c7fb0143921c886]::util::run_in_thread_pool_with_globals<rustc_interface[4c7fb0143921c886]::interface::run_compiler<core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>, rustc_driver[722cf9eaf3bfc82d]::run_compiler::{closure#1}>::{closure#0}, core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1bdb63ea3d2475fb]::result::Result<(), rustc_errors[b3a8aea407dcd03e]::ErrorGuaranteed>>::{closure#1} as core[1bdb63ea3d2475fb]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7fefe8f92803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1a6bfb8adf6f597d
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/alloc/src/boxed.rs:2000:9
  63:     0x7fefe8f92803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9280d6ff8fccf766
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/alloc/src/boxed.rs:2000:9
  64:     0x7fefe8f92803 - std::sys::unix::thread::Thread::new::thread_start::h0287d74c430bd71c
                               at /rustc/b364405cc3ef6a5786bfc27bf9b4c187f70f9201/library/std/src/sys/unix/thread.rs:108:17
  65:     0x7fefe44e16db - start_thread
  66:     0x7fefe420a61f - clone
  67:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-beta.3 (b364405cc 2022-12-27) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C linker=i686-w64-mingw32-gcc -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C split-debuginfo=off -C prefer-dynamic -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [specialization_graph_of] building specialization graph of trait `collections::SpecExtend`
#1 [coherent_trait] coherence checking all impls of trait `collections::SpecExtend`
#2 [analysis] running analysis passes on this crate
Some errors have detailed explanations: E0412, E0747.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `alloc` due to 20 previous errors
Build completed unsuccessfully in 0:00:20
