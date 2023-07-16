
--------------------------------------------------------------------------------
Ir         
--------------------------------------------------------------------------------
89,201,296  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir              file:function
--------------------------------------------------------------------------------
-1,370,843,599  ???:<rustc_trait_selection::traits::select::SelectionContext>::match_impl
 1,206,508,602  ???:<rustc_middle::ty::context::TyCtxt>::for_each_relevant_impl::<<rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates_from_impls::{closure
   266,225,359  ???:<rustc_middle::ty::context::TyCtxt>::bound_impl_trait_ref
   -81,726,346  ???:rustc_trait_selection::traits::coherence::overlap_within_probe
    71,405,351  ???:<rustc_middle::ty::context::TyCtxt>::bound_type_of
    -4,271,624  ???:<rustc_infer::infer::equate::Equate as rustc_middle::ty::relate::TypeRelation>::tys
     1,663,549  ???:<rustc_infer::infer::InferCtxt>::probe::<(), <rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates_from_impls::{closure
       689,469  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::ArenaCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::Generics>, &rustc_middle::ty::generics::Generics, rustc_middle::ty::query::copy<&rustc_middle::ty::generics::Generics>>
       615,300  ???:<&mut rustc_typeck::check::compare_method::check_type_bounds::{closure
      -578,580  ???:<core::iter::adapters::map::Map<core::slice::iter::Iter<(rustc_middle::ty::Predicate, rustc_span::span_encoding::Span)>, rustc_typeck::check::compare_method::check_type_bounds::{closure
      -351,062  ???:<rustc_typeck::check::writeback::EraseEarlyRegions as rustc_middle::ty::fold::TypeFolder>::fold_ty
      -342,817  ???:<rustc_trait_selection::traits::select::SelectionContext>::rematch_impl
       339,609  ???:<alloc::vec::Vec<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>> as alloc::vec::spec_from_iter::SpecFromIter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, core::iter::adapters::map::Map<core::iter::adapters::map::Map<rustc_middle::ty::sty::EarlyBinderIter<core::slice::iter::Iter<(rustc_middle::ty::Predicate, rustc_span::span_encoding::Span)>>, rustc_typeck::check::compare_method::check_type_bounds::{closure
       325,105  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
       298,967  /build/glibc-sMfBJT/glibc-2.31/string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
       251,813  ???:rustc_hir::intravisit::walk_ty::<rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor>
      -247,829  ???:<rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor as rustc_hir::intravisit::Visitor>::visit_ty
      -226,220  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_type_vars_in_body
       209,903  ???:<rustc_typeck::check::writeback::WritebackCx>::visit_user_provided_tys
      -207,656  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types::{closure
      -195,962  ???:<rustc_middle::hir::map::Map>::par_visit_all_item_likes::<rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor>
       195,961  ???:rustc_data_structures::sync::par_for_each_in::<&alloc::vec::Vec<rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>>, <rustc_middle::hir::map::Map>::par_visit_all_item_likes<rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor>::{closure
      -194,033  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
      -185,113  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::subst::SubstFolder>
       184,725  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::GenericPredicates>, rustc_middle::ty::generics::GenericPredicates, rustc_middle::ty::query::copy<rustc_middle::ty::generics::GenericPredicates>>
      -181,539  ???:rustc_typeck::check::compare_method::compare_ty_impl
      -174,855  ???:rustc_typeck::check::compare_method::compare_generic_param_kinds::{closure
      -168,012  ???:rustc_typeck::check::compare_method::check_type_bounds
       163,475  ???:_rjem_je_arena_ralloc
       163,265  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_middle::ty::subst::SubstFolder>
       128,271  ???:<rustc_typeck::check::writeback::WritebackCx>::visit_opaque_types
       114,649  ???:<core::result::Result<rustc_middle::ty::subst::GenericArg, rustc_middle::ty::error::TypeError> as rustc_middle::ty::context::InternIteratorElement<rustc_middle::ty::subst::GenericArg, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>>::intern_with::<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>, core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>>>, rustc_middle::ty::relate::relate_substs_with_variances<rustc_infer::infer::sub::Sub>::{closure
      -113,502  ???:<alloc::vec::Vec<u64> as core::ops::deref::DerefMut>::deref_mut
       110,644  ???:<rustc_middle::ty::context::TyCtxt>::bound_fn_sig
       101,468  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_typeck::check::writeback::EraseEarlyRegions>
       101,381  ???:<rustc_middle::ty::sty::EarlyBinder<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig>> as rustc_middle::ty::subst::Subst>::subst
       -98,452  ???:<&mut rustc_middle::ty::relate::relate_substs_with_variances<rustc_infer::infer::combine::Generalizer>::{closure
       -98,324  ???:rustc_typeck::check::compare_method::compare_number_of_generics
        94,148  ???:_rjem_je_arena_ralloc_no_move
       -91,628  ???:<rustc_infer::infer::InferCtxtBuilder>::enter::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_typeck::check::compare_method::check_type_bounds::{closure
