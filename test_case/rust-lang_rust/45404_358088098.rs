
let mut is_trait = None;
// giannicic: New variable
let mut is_impl_trait = None;

let icx = ItemCtxt::new(tcx, def_id);
let no_generics = hir::Generics::empty();
let (ast_generics, opt_inputs) = match node {
    NodeTraitItem(item) => {
        match item.node {
            TraitItemKind::Method(ref sig, _) => (&item.generics, Some(&sig.decl.inputs)),
            _ => (&item.generics, None)
        }
    }

    NodeImplItem(item) => {
        match item.node {
            ImplItemKind::Method(ref sig, _) => (&item.generics, Some(&sig.decl.inputs)),
            _ => (&item.generics, None)
        }
    }

    NodeItem(item) => {
        match item.node {
            ItemFn(ref decl, .., ref generics, _) => (generics, Some(&decl.inputs)),

            ItemImpl(_, _, defaultness, ref generics, ..) => {
                // giannicic: I've added this block to get the TraitRef
                if defaultness.is_default() {
                    let is_impl_trait_ref = tcx.impl_trait_ref(tcx.hir.local_def_id(item.id));
                    is_impl_trait = Some(ty::TraitRef {
                        def_id: is_impl_trait_ref.unwrap().def_id,
                        substs: Substs::identity_for_item(tcx, is_impl_trait_ref.unwrap().def_id)
                    });
                }
                (generics, None)
            }
            ItemTy(_, ref generics) |
            ItemEnum(_, ref generics) |
            ItemStruct(_, ref generics) |
            ItemUnion(_, ref generics) => {
                (generics, None)
            }

            ItemTrait(_, _, ref generics, .., ref items) => {
                is_trait = Some((ty::TraitRef {
                    def_id,
                    substs: Substs::identity_for_item(tcx, def_id)
                }, items));
                (generics, None)
            }

            _ => (&no_generics, None)
        }
    }

    NodeForeignItem(item) => {
        match item.node {
            ForeignItemStatic(..) => (&no_generics, None),
            ForeignItemFn(ref decl, _, ref generics) => (generics, Some(&decl.inputs)),
            ForeignItemType => (&no_generics, None),
        }
    }

    NodeTy(&Ty { node: TyImplTraitExistential(ref exist_ty, _), span, .. }) => {
        let substs = Substs::identity_for_item(tcx, def_id);
        let anon_ty = tcx.mk_anon(def_id, substs);

        debug!("explicit_predicates_of: anon_ty={:?}", anon_ty);

        // Collect the bounds, i.e. the `A+B+'c` in `impl A+B+'c`.
        let bounds = compute_bounds(&icx,
                                    anon_ty,
                                    &exist_ty.bounds,
                                    SizedByDefault::Yes,
                                    span);

        debug!("explicit_predicates_of: bounds={:?}", bounds);

        let predicates = bounds.predicates(tcx, anon_ty);

        debug!("explicit_predicates_of: predicates={:?}", predicates);

        return ty::GenericPredicates {
            parent: None,
            predicates: predicates
        };
    }

    _ => (&no_generics, None)
};

let generics = tcx.generics_of(def_id);
let parent_count = generics.parent_count() as u32;
let has_own_self = generics.has_self && parent_count == 0;

let mut predicates = vec![];

// Below we'll consider the bounds on the type parameters (including `Self`)
// and the explicit where-clauses, but to get the full set of predicates
// on a trait we need to add in the supertrait bounds and bounds found on
// associated types.
if let Some((trait_ref, _)) = is_trait {
    predicates = tcx.super_predicates_of(def_id).predicates;

    // Add in a predicate that `Self:Trait` (where `Trait` is the
    // current trait).  This is needed for builtin bounds.
    predicates.push(trait_ref.to_poly_trait_ref().to_predicate());
}

// giannicic: And I've used the TraitRef to insert the predicate
if let Some(trait_ref) = is_impl_trait {
    predicates.push(trait_ref.to_poly_trait_ref().to_predicate());
}
