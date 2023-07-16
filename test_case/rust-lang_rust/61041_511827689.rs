
error: internal compiler error: src/librustc/ty/subst.rs:557: type parameter `Self/#0` (Self/0) out of range when substituting (root type=Some(<Self as float::Float>::Int)) substs=[]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:584:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /home/r/src/rust/rustc/src/liballoc/boxed.rs:780
   7: rustc::util::common::panic_hook
             at src/librustc/util/common.rs:40
   8: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   9: std::panicking::begin_panic
             at /home/r/src/rust/rustc/src/libstd/panicking.rs:411
  10: rustc_errors::Handler::span_bug
             at /home/r/src/rust/rustc/<::std::macros::panic macros>:4
  11: rustc::util::bug::opt_span_bug_fmt::{{closure}}
             at src/librustc/util/bug.rs:35
  12: rustc::ty::context::tls::with_opt::{{closure}}
             at src/librustc/ty/context.rs:1994
  13: rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1944
  14: rustc::ty::context::tls::with_opt
             at src/librustc/ty/context.rs:1994
  15: rustc::util::bug::opt_span_bug_fmt
             at src/librustc/util/bug.rs:32
  16: rustc::util::bug::span_bug_fmt
             at src/librustc/util/bug.rs:23
  17: rustc::ty::subst::SubstFolder::ty_for_param
             at src/librustc/ty/subst.rs:557
  18: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty
             at src/librustc/ty/subst.rs:506
  19: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::fold_with
             at src/librustc/ty/structural_impls.rs:1077
  20: <rustc::ty::subst::Kind as rustc::ty::fold::TypeFoldable>::super_fold_with
             at src/librustc/ty/subst.rs:154
  21: rustc::ty::fold::TypeFoldable::fold_with
             at src/librustc/ty/fold.rs:50
  22: rustc::ty::subst::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::List<rustc::ty::subst::Kind>>::super_fold_with::{{closure}}
             at src/librustc/ty/subst.rs:386
  23: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
             at /home/r/src/rust/rustc/src/libcore/ops/function.rs:279
  24: core::option::Option<T>::map
             at /home/r/src/rust/rustc/src/libcore/option.rs:447
  25: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next
             at /home/r/src/rust/rustc/src/libcore/iter/adapters/mod.rs:570
  26: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
             at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-0.6.10/lib.rs:1357
  27: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
             at /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-0.6.10/lib.rs:1342
  28: core::iter::traits::iterator::Iterator::collect
             at /home/r/src/rust/rustc/src/libcore/iter/traits/iterator.rs:1464
  29: rustc::ty::subst::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::List<rustc::ty::subst::Kind>>::super_fold_with
             at src/librustc/ty/subst.rs:386
  30: rustc::ty::fold::TypeFoldable::fold_with
             at src/librustc/ty/fold.rs:50
  31: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for rustc::ty::sty::ProjectionTy>::super_fold_with
             at src/librustc/macros.rs:340
  32: rustc::ty::fold::TypeFoldable::fold_with
             at src/librustc/ty/fold.rs:50
  33: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
             at src/librustc/ty/structural_impls.rs:1048
  34: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty
             at src/librustc/ty/subst.rs:509
  35: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::fold_with
             at src/librustc/ty/structural_impls.rs:1077
  36: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::sty::Const>::super_fold_with
             at src/librustc/ty/structural_impls.rs:1346
  37: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_const
             at src/librustc/ty/subst.rs:530
  38: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::sty::Const>::fold_with
             at /home/r/src/rust/rustc/src/librustc/ty/structural_impls.rs:1355
  39: <T as rustc::ty::subst::Subst>::subst_spanned
             at /home/r/src/rust/rustc/src/librustc/ty/subst.rs:427
  40: rustc::ty::subst::Subst::subst
             at /home/r/src/rust/rustc/src/librustc/ty/subst.rs:413
  41: rustc_mir::interpret::eval_context::InterpCx<M>::monomorphize_with_substs
             at src/librustc_mir/interpret/eval_context.rs:373
  42: rustc_mir::interpret::eval_context::InterpCx<M>::monomorphize
             at src/librustc_mir/interpret/eval_context.rs:357
  43: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_const_to_op
             at src/librustc_mir/interpret/operand.rs:532
  44: rustc_mir::transform::const_prop::ConstPropagator::eval_constant
             at src/librustc_mir/transform/const_prop.rs:369
  45: rustc_mir::transform::const_prop::ConstPropagator::eval_operand
             at src/librustc_mir/transform/const_prop.rs:437
  46: rustc_mir::transform::const_prop::ConstPropagator::const_prop
             at src/librustc_mir/transform/const_prop.rs:452
  47: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_statement
             at src/librustc_mir/transform/const_prop.rs:783
  48: rustc::mir::visit::MutVisitor::super_basic_block_data
             at /home/r/src/rust/rustc/src/librustc/mir/visit.rs:314
  49: rustc::mir::visit::MutVisitor::visit_basic_block_data
             at /home/r/src/rust/rustc/src/librustc/mir/visit.rs:81
  50: rustc::mir::visit::MutVisitor::super_body
             at /home/r/src/rust/rustc/src/librustc/mir/visit.rs:272
  51: rustc::mir::visit::MutVisitor::visit_body
             at /home/r/src/rust/rustc/src/librustc/mir/visit.rs:75
  52: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
             at src/librustc_mir/transform/const_prop.rs:99
  53: rustc_mir::transform::run_passes::{{closure}}::{{closure}}
             at src/librustc_mir/transform/mod.rs:172
  54: rustc_mir::transform::run_passes::{{closure}}
             at src/librustc_mir/transform/mod.rs:179
  55: rustc_mir::transform::run_passes
             at src/librustc_mir/transform/mod.rs:185
  56: rustc_mir::transform::optimized_mir
             at src/librustc_mir/transform/mod.rs:244
  57: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:997
  58: rustc::ty::query::__query_compute::optimized_mir
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:948
  59: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:989
  60: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/r/src/rust/rustc/src/librustc/dep_graph/graph.rs:334
  61: rustc::dep_graph::graph::DepGraph::with_task
             at /home/r/src/rust/rustc/src/librustc/dep_graph/graph.rs:202
  62: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:556
  63: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:275
  64: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1866
  65: rustc::ty::context::tls::set_tlv
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1799
  66: rustc::ty::context::tls::enter_context
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1865
  67: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:274
  68: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1972
  69: rustc::ty::context::tls::with_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1955
  70: rustc::ty::context::tls::with_context_opt
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1944
  71: rustc::ty::context::tls::with_context
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1955
  72: rustc::ty::context::tls::with_related_context
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1968
  73: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:263
  74: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:548
  75: rustc::ty::query::plumbing::with_diagnostics
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:209
  76: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:547
  77: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:376
  78: rustc::ty::query::TyCtxtAt::optimized_mir
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:1074
  79: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::optimized_mir
             at /home/r/src/rust/rustc/src/librustc/ty/query/plumbing.rs:1066
  80: rustc_metadata::encoder::EncodeContext::encode_optimized_mir
             at src/librustc_metadata/encoder.rs:1031
  81: rustc_metadata::encoder::EncodeContext::encode_info_for_item
             at src/librustc_metadata/encoder.rs:0
  82: core::ops::function::FnOnce::call_once
             at /home/r/src/rust/rustc/src/libcore/ops/function.rs:231
  83: rustc_metadata::encoder::EncodeContext::record
             at src/librustc_metadata/encoder.rs:314
  84: <rustc_metadata::encoder::EncodeContext as rustc::hir::intravisit::Visitor>::visit_item
             at src/librustc_metadata/encoder.rs:1663
  85: <rustc::hir::itemlikevisit::DeepVisitor<V> as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
             at /home/r/src/rust/rustc/src/librustc/hir/itemlikevisit.rs:70
  86: rustc::hir::Crate::visit_all_item_likes
             at /home/r/src/rust/rustc/src/librustc/hir/mod.rs:782
  87: rustc_metadata::encoder::EncodeContext::encode_info_for_items
             at src/librustc_metadata/encoder.rs:325
  88: rustc_metadata::encoder::EncodeContext::encode_crate_root
             at src/librustc_metadata/encoder.rs:428
  89: rustc_metadata::encoder::encode_metadata::{{closure}}
             at src/librustc_metadata/encoder.rs:1893
  90: rustc::dep_graph::graph::DepGraph::with_ignore::{{closure}}::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/dep_graph/graph.rs:159
  91: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1866
  92: rustc::ty::context::tls::set_tlv
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1799
  93: rustc::ty::context::tls::enter_context
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1865
  94: rustc::dep_graph::graph::DepGraph::with_ignore::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/dep_graph/graph.rs:158
  95: rustc::ty::context::tls::with_context::{{closure}}
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1955
  96: rustc::ty::context::tls::with_context_opt
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1944
  97: rustc::ty::context::tls::with_context
             at /home/r/src/rust/rustc/src/librustc/ty/context.rs:1955
  98: rustc::dep_graph::graph::DepGraph::with_ignore
             at /home/r/src/rust/rustc/src/librustc/dep_graph/graph.rs:152
  99: rustc_metadata::encoder::encode_metadata
             at src/librustc_metadata/encoder.rs:1875
  100: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
             at src/librustc_metadata/cstore_impl.rs:554
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [optimized_mir] processing `float::conv::__fixsfsi`
end of query stack
error: aborting due to previous error
