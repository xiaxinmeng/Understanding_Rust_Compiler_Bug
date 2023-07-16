
Samples: 22M of event 'cycles:uppp', Event count (approx.): 22349476896140
Overhead  Command          Shared Object                                 Symbol
  21.14%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::traits::select::SelectionContext::evaluate_stack
   6.54%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
   4.53%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::traits::select::SelectionContext::evaluate_predicate_recursively
   3.03%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::ty::context::TyCtxt::_intern_substs
   2.51%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::infer::InferCtxt::start_snapshot
   2.48%  rustc            libc-2.28.so                                  [.] __memmove_avx_unaligned_erms
   2.31%  rustc            libc-2.28.so                                  [.] _int_malloc
   1.63%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::traits::select::SelectionContext::evaluate_predicates_recursively
   1.59%  rustc            libc-2.28.so                                  [.] _int_free
   1.34%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::infer::InferCtxt::in_snapshot
   1.25%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
   1.22%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::traits::select::SelectionContext::confirm_candidate
   1.10%  rustc            libc-2.28.so                                  [.] malloc
   1.00%  rustc            librustc-e9b5ca1e357beddc.so                  [.] <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
   0.97%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::infer::InferCtxt::commit_from
   0.92%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::traits::select::SelectionContext::candidate_from_obligation
   0.88%  rustc            librustc-e9b5ca1e357beddc.so                  [.] <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
   0.86%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::ty::context::CtxtInterners::intern_ty
   0.82%  rustc            librustc-e9b5ca1e357beddc.so                  [.] <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
   0.81%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
   0.70%  rustc            libc-2.28.so                                  [.] realloc
   0.67%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::ty::fold::<impl rustc::ty::context::TyCtxt>::replace_bound_vars
   0.65%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::traits::project::normalize_with_depth
   0.62%  rustc            librustc-e9b5ca1e357beddc.so                  [.] <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
   0.62%  rustc            librustc-e9b5ca1e357beddc.so                  [.] <rustc::traits::ObligationCause as core::clone::Clone>::clone
   0.60%  rustc            libLLVM-8-rust-1.35.0-beta.so                 [.] llvm::GlobalsAAResult::DeletionCallbackHandle::deleted
   0.58%  rustc            librustc-e9b5ca1e357beddc.so                  [.] <T as rustc::ty::context::InternIteratorElement<T,R>>::intern_with
   0.58%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::infer::InferCtxt::probe
   0.57%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::traits::select::SelectionContext::impl_or_trait_obligations
   0.49%  rustc            rustc                                         [.] free
   0.48%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::infer::InferCtxt::in_snapshot
   0.45%  rustc            librustc-e9b5ca1e357beddc.so                  [.] rustc::traits::select::SelectionContext::collect_predicates_for_types
   0.45%  rustc            librustc-e9b5ca1e357beddc.so                  [.] <rustc::traits::ObligationCause as core::clone::Clone>::clone
   0.40%  rustc            libLLVM-8-rust-1.35.0-beta.so                 [.] combineInstructionsOverFunction
   0.38%  rustc            libLLVM-8-rust-1.35.0-beta.so                 [.] llvm::ValueHandleBase::AddToUseList
   0.37%  rustc            rustc                                         [.] malloc
