rust
                ty::ExistentialPredicate::Projection(projection) => {
                    let assoc_item = self.tcx.associated_item(projection.item_def_id);

                    let last_trait_ref = last_trait_ref
                        .expect("trait predicate must come before projection predicate");
                    assert_eq!(last_trait_ref.bound_vars(), predicate.bound_vars());
                    // Use a type that can't appear in defaults of type parameters.
                    let dummy_self = self.tcx.mk_ty_infer(ty::FreshTy(0));
                    let assoc_item_parent_trait =
                        traits::supertraits(self.tcx, projection.trait_ref(self.tcx).with_self_ty(self.tcx, dummy_self)).find(|r| {
                            self.tcx
                                .associated_items(r.def_id())
                                .find_by_name_and_kind(
                                    self.tcx,
                                    assoc_item.ident,
                                    assoc_item.kind,
                                    r.def_id(),
                                )
                                .is_some()
                        }).unwrap();
                    assert_eq!(last_trait_ref.skip_binder(), ty::ExistentialTraitRef::erase_self_ty(self.tcx, assoc_item_parent_trait.skip_binder()));

                    let name = assoc_item.ident;
                    self.push("p");
                    self.push_ident(&name.as_str());
                    self = projection.ty.print(self)?;
                }
