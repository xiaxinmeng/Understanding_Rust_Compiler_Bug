
> traits::supertraits(tcx, trait_ref).filter(|r| {
>   tcx
>     .associated_items(r.def_id())
>     .find_by_name_and_kind(tcx, assoc_name, ty::AssocKind::Type, r.def_id())
>     .is_some()
> }).next()
> 