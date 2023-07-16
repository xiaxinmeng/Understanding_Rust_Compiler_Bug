
#0  0x00007ffff4850381 in rustc::ty::context::TyCtxt::_intern_substs () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#1  0x00007ffff4657d96 in rustc::ty::subst::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::Slice<rustc::ty::subst::Kind<'tcx>>>::super_fold_with () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#2  0x00007ffff460d165 in rustc::ty::fold::TypeFoldable::fold_with () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#3  0x00007ffff45f1c85 in rustc::traits::select::SelectionContext::impl_or_trait_obligations::{{closure}} ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#4  0x00007ffff4426dbb in <core::iter::FlatMap<I, U, F> as core::iter::iterator::Iterator>::next ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#5  0x00007ffff42b8f27 in <alloc::vec::Vec<T>>::extend_desugared () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#6  0x00007ffff441eea9 in <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#7  0x00007ffff45f1a83 in rustc::traits::select::SelectionContext::impl_or_trait_obligations ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#8  0x00007ffff45f0fd2 in rustc::traits::select::SelectionContext::confirm_impl_candidate::{{closure}} ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#9  0x00007ffff453559c in rustc::infer::InferCtxt::in_snapshot () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#10 0x00007ffff45e9e4f in rustc::traits::select::SelectionContext::confirm_candidate ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#11 0x00007ffff45dee2d in rustc::traits::select::SelectionContext::select () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#12 0x00007ffff45d68df in rustc::traits::project::confirm_candidate () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#13 0x00007ffff45d5413 in rustc::traits::project::opt_normalize_projection_type ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#14 0x00007ffff45d2c34 in rustc::traits::project::normalize_projection_type ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#15 0x00007ffff45d259e in <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#16 0x00007ffff46812ab in rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for rustc::ty::Predicate<'tcx>>::super_fold_with () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#17 0x00007ffff45f1d87 in rustc::traits::select::SelectionContext::impl_or_trait_obligations::{{closure}} ()
---Type <return> to continue, or q <return> to quit---
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#18 0x00007ffff4426dbb in <core::iter::FlatMap<I, U, F> as core::iter::iterator::Iterator>::next ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#19 0x00007ffff441ec4c in <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#20 0x00007ffff45f1a83 in rustc::traits::select::SelectionContext::impl_or_trait_obligations ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#21 0x00007ffff45f0fd2 in rustc::traits::select::SelectionContext::confirm_impl_candidate::{{closure}} ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#22 0x00007ffff453559c in rustc::infer::InferCtxt::in_snapshot () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#23 0x00007ffff45e9e4f in rustc::traits::select::SelectionContext::confirm_candidate ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#24 0x00007ffff45dee2d in rustc::traits::select::SelectionContext::select () from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#25 0x00007ffff45d639c in rustc::traits::project::assemble_candidates_from_impls::{{closure}} ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#26 0x00007ffff45d6219 in rustc::traits::project::assemble_candidates_from_impls ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#27 0x00007ffff45d4470 in rustc::traits::project::opt_normalize_projection_type ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
#28 0x00007ffff45d2c34 in rustc::traits::project::normalize_projection_type ()
   from /usr/bin/../lib/../lib/librustc-5407144f5a31b71b.so
