
error: internal compiler error: src/librustc/ty/relate.rs:581: var types encountered in super_relate_consts: Const { ty: std::array::IntoIter<T, N>, val: Infer(Var(_#0c)) } Const { ty: std::array::IntoIter<T, N>, val: Infer(Var(_#0c)) }

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:646:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /home/lukas/dev/rust/src/liballoc/boxed.rs:780
   7: rustc::util::common::panic_hook
             at src/librustc/util/common.rs:40
   8: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   9: std::panicking::begin_panic
             at /home/lukas/dev/rust/src/libstd/panicking.rs:411
  10: rustc_errors::Handler::bug
             at src/librustc_errors/lib.rs:646
  11: rustc::util::bug::opt_span_bug_fmt::{{closure}}
             at src/librustc/util/bug.rs:36
  12: rustc::ty::context::tls::with_opt::{{closure}}
             at src/librustc/ty/context.rs:1994
  13: rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1944
  14: rustc::ty::context::tls::with_opt
             at src/librustc/ty/context.rs:1994
  15: rustc::util::bug::opt_span_bug_fmt
             at src/librustc/util/bug.rs:32
  16: rustc::util::bug::bug_fmt
             at src/librustc/util/bug.rs:12
  17: rustc::ty::relate::super_relate_consts
             at /home/lukas/dev/rust/src/librustc/macros.rs:44
  18: <rustc::infer::nll_relate::TypeGeneralizer<D> as rustc::ty::relate::TypeRelation>::consts
             at /home/lukas/dev/rust/src/librustc/infer/nll_relate/mod.rs:996
  19: <&rustc::ty::sty::Const as rustc::ty::relate::Relate>::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:692
  20: rustc::ty::relate::TypeRelation::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:43
  21: <rustc::ty::subst::Kind as rustc::ty::relate::Relate>::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:744
  22: rustc::ty::relate::TypeRelation::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:43
  23: <rustc::infer::nll_relate::TypeGeneralizer<D> as rustc::ty::relate::TypeRelation>::relate_with_variance
             at /home/lukas/dev/rust/src/librustc/infer/nll_relate/mod.rs:861
  24: rustc::ty::relate::relate_substs::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:141
  25: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
             at /home/lukas/dev/rust/src/libcore/ops/function.rs:283
  26: core::option::Option<T>::map
             at /home/lukas/dev/rust/src/libcore/option.rs:447
  27: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next
             at /home/lukas/dev/rust/src/libcore/iter/adapters/mod.rs:570
  28: <<core::result::Result<V,E> as core::iter::traits::collect::FromIterator<core::result::Result<A,E>>>::from_iter::Adapter<Iter,E> as core::iter::traits::iterator::Iterator>::next
             at /home/lukas/dev/rust/src/libcore/result.rs:1323
  29: <&mut I as core::iter::traits::iterator::Iterator>::next
             at /home/lukas/dev/rust/src/libcore/iter/traits/iterator.rs:2706
  30: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-0.6.10/lib.rs:1357
  31: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-0.6.10/lib.rs:1342
  32: <core::result::Result<V,E> as core::iter::traits::collect::FromIterator<core::result::Result<A,E>>>::from_iter
             at /home/lukas/dev/rust/src/libcore/result.rs:1340
  33: core::iter::traits::iterator::Iterator::collect
             at /home/lukas/dev/rust/src/libcore/iter/traits/iterator.rs:1464
  34: <core::result::Result<T,E> as rustc::ty::context::InternIteratorElement<T,R>>::intern_with
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:2886
  35: <I as rustc::ty::context::InternAs<[T],R>>::intern_with
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:2858
  36: rustc::ty::context::TyCtxt::mk_substs
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:2717
  37: rustc::ty::relate::relate_substs
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:144
  38: rustc::ty::relate::TypeRelation::relate_item_substs
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:61
  39: rustc::ty::relate::super_relate_tys
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:392
  40: <rustc::infer::nll_relate::TypeGeneralizer<D> as rustc::ty::relate::TypeRelation>::tys
             at /home/lukas/dev/rust/src/librustc/infer/nll_relate/mod.rs:945
  41: <&rustc::ty::TyS as rustc::ty::relate::Relate>::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:336
  42: rustc::ty::relate::TypeRelation::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:43
  43: rustc::infer::nll_relate::TypeRelating<D>::generalize_value
             at /home/lukas/dev/rust/src/librustc/infer/nll_relate/mod.rs:395
  44: rustc::infer::nll_relate::TypeRelating<D>::relate_ty_var
             at /home/lukas/dev/rust/src/librustc/infer/nll_relate/mod.rs:348
  45: <rustc::infer::nll_relate::TypeRelating<D> as rustc::ty::relate::TypeRelation>::tys
             at /home/lukas/dev/rust/src/librustc/infer/nll_relate/mod.rs:558
  46: <&rustc::ty::TyS as rustc::ty::relate::Relate>::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:336
  47: rustc::ty::relate::TypeRelation::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:43
  48: <rustc::ty::subst::Kind as rustc::ty::relate::Relate>::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:741
  49: rustc::ty::relate::TypeRelation::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:43
  50: <rustc::infer::nll_relate::TypeRelating<D> as rustc::ty::relate::TypeRelation>::relate_with_variance
             at /home/lukas/dev/rust/src/librustc/infer/nll_relate/mod.rs:532
  51: rustc::ty::relate::relate_substs::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:141
  52: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
             at /home/lukas/dev/rust/src/libcore/ops/function.rs:283
  53: core::option::Option<T>::map
             at /home/lukas/dev/rust/src/libcore/option.rs:447
  54: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next
             at /home/lukas/dev/rust/src/libcore/iter/adapters/mod.rs:570
  55: <<core::result::Result<V,E> as core::iter::traits::collect::FromIterator<core::result::Result<A,E>>>::from_iter::Adapter<Iter,E> as core::iter::traits::iterator::Iterator>::next
             at /home/lukas/dev/rust/src/libcore/result.rs:1323
  56: <&mut I as core::iter::traits::iterator::Iterator>::next
             at /home/lukas/dev/rust/src/libcore/iter/traits/iterator.rs:2706
  57: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-0.6.10/lib.rs:1357
  58: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-0.6.10/lib.rs:1342
  59: <core::result::Result<V,E> as core::iter::traits::collect::FromIterator<core::result::Result<A,E>>>::from_iter
             at /home/lukas/dev/rust/src/libcore/result.rs:1340
  60: core::iter::traits::iterator::Iterator::collect
             at /home/lukas/dev/rust/src/libcore/iter/traits/iterator.rs:1464
  61: <core::result::Result<T,E> as rustc::ty::context::InternIteratorElement<T,R>>::intern_with
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:2886
  62: <I as rustc::ty::context::InternAs<[T],R>>::intern_with
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:2858
  63: rustc::ty::context::TyCtxt::mk_substs
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:2717
  64: rustc::ty::relate::relate_substs
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:144
  65: <rustc::ty::sty::TraitRef as rustc::ty::relate::Relate>::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:286
  66: rustc::ty::relate::TypeRelation::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:43
  67: <rustc::ty::TraitPredicate as rustc::ty::relate::Relate>::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:766
  68: rustc::ty::relate::TypeRelation::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:43
  69: <rustc::traits::WhereClause as rustc::ty::relate::Relate>::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:793
  70: rustc::ty::relate::TypeRelation::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:43
  71: <rustc::traits::DomainGoal as rustc::ty::relate::Relate>::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:857
  72: rustc::ty::relate::TypeRelation::relate
             at /home/lukas/dev/rust/src/librustc/ty/relate.rs:43
  73: rustc_traits::chalk_context::unify::unify
             at src/librustc_traits/chalk_context/unify.rs:31
  74: rustc_traits::chalk_context::resolvent_ops::<impl chalk_engine::context::ResolventOps<rustc_traits::chalk_context::ChalkArenas,rustc_traits::chalk_context::ChalkArenas> for rustc_traits::chalk_context::ChalkInferenceContext>::resolvent_clause::{{closure}}
             at src/librustc_traits/chalk_context/resolvent_ops.rs:56
  75: rustc::infer::InferCtxt::probe
             at /home/lukas/dev/rust/src/librustc/infer/mod.rs:844
  76: rustc_traits::chalk_context::resolvent_ops::<impl chalk_engine::context::ResolventOps<rustc_traits::chalk_context::ChalkArenas,rustc_traits::chalk_context::ChalkArenas> for rustc_traits::chalk_context::ChalkInferenceContext>::resolvent_clause
             at src/librustc_traits/chalk_context/resolvent_ops.rs:42
  77: chalk_engine::logic::<impl chalk_engine::forest::Forest<C,CO>>::push_initial_strands_instantiated
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/logic.rs:773
  78: <chalk_engine::logic::<impl chalk_engine::forest::Forest<C,CO>>::push_initial_strands::PushInitialStrandsInstantiated<C,CO> as chalk_engine::context::WithInstantiatedUCanonicalGoal<C>>::with
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/logic.rs:753
  79: <rustc_traits::chalk_context::ChalkContext as chalk_engine::context::ContextOps<rustc_traits::chalk_context::ChalkArenas>>::instantiate_ucanonical_goal::{{closure}}
             at src/librustc_traits/chalk_context/mod.rs:224
  80: rustc::infer::InferCtxtBuilder::enter_with_canonical::{{closure}}
             at /home/lukas/dev/rust/src/librustc/infer/mod.rs:511
  81: rustc::infer::InferCtxtBuilder::enter::{{closure}}
             at /home/lukas/dev/rust/src/librustc/infer/mod.rs:522
  82: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1643
  83: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1866
  84: rustc::ty::context::tls::set_tlv
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1799
  85: rustc::ty::context::tls::enter_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1865
  86: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1642
  87: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1972
  88: rustc::ty::context::tls::with_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1955
  89: rustc::ty::context::tls::with_context_opt
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1944
  90: rustc::ty::context::tls::with_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1955
  91: rustc::ty::context::tls::with_related_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1968
  92: rustc::ty::context::GlobalCtxt::enter_local
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1634
  93: rustc::infer::InferCtxtBuilder::enter
             at /home/lukas/dev/rust/src/librustc/infer/mod.rs:521
  94: rustc::infer::InferCtxtBuilder::enter_with_canonical
             at /home/lukas/dev/rust/src/librustc/infer/mod.rs:508
  95: <rustc_traits::chalk_context::ChalkContext as chalk_engine::context::ContextOps<rustc_traits::chalk_context::ChalkArenas>>::instantiate_ucanonical_goal
             at src/librustc_traits/chalk_context/mod.rs:220
  96: chalk_engine::logic::<impl chalk_engine::forest::Forest<C,CO>>::push_initial_strands
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/logic.rs:731
  97: chalk_engine::logic::<impl chalk_engine::forest::Forest<C,CO>>::get_or_create_table_for_ucanonical_goal
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/logic.rs:713
  98: chalk_engine::logic::<impl chalk_engine::forest::Forest<C,CO>>::get_or_create_table_for_subgoal
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/logic.rs:683
  99: chalk_engine::logic::<impl chalk_engine::forest::Forest<C,CO>>::pursue_strand
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/logic.rs:497
  100: <chalk_engine::logic::PursueStrand<C,CO> as chalk_engine::logic::WithInstantiatedStrand<C,CO>>::with
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/logic.rs:1327
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [evaluate_goal] evaluating trait selection obligation `^1_0: std::clone::Clone`
#1 [typeck_tables_of] processing `bar`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
