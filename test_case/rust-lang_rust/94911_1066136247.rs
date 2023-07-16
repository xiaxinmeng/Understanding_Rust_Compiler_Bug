plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs at line 508:
             // higher-ranked things.
             // Prevent, e.g., `dyn Iterator<Item = str>`.
             for bound in self.tcx().item_bounds(assoc_type) {
-                let subst_bound = if defs.count() == 0 {
-                    bound.subst(tcx, trait_predicate.trait_ref.substs)
-                } else {
-                    let mut substs = smallvec::SmallVec::with_capacity(defs.count());
-                    substs.extend(trait_predicate.trait_ref.substs.iter());
-                    let mut bound_vars: smallvec::SmallVec<[ty::BoundVariableKind; 8]> =
-                        smallvec::SmallVec::with_capacity(bound.kind().bound_vars().len() + defs.count());
-                    bound_vars.extend(bound.kind().bound_vars().into_iter());
-                    InternalSubsts::fill_single(&mut substs, defs, &mut |param, _| match param.kind {
-                        GenericParamDefKind::Type { .. } => {
-                            let kind = ty::BoundTyKind::Param(param.name);
-                            let bound_var = ty::BoundVariableKind::Ty(kind);
-                            bound_vars.push(bound_var);
-                            tcx.mk_ty(ty::Bound(
-                                ty::INNERMOST,
-                                ty::BoundTy { var: ty::BoundVar::from_usize(bound_vars.len() - 1), kind },
-                            .into()
-                        }
-                        GenericParamDefKind::Lifetime => {
-                        GenericParamDefKind::Lifetime => {
-                            let kind = ty::BoundRegionKind::BrNamed(param.def_id, param.name);
-                            let bound_var = ty::BoundVariableKind::Region(kind);
-                            bound_vars.push(bound_var);
-                            tcx.mk_region(ty::ReLateBound(
-                                ty::INNERMOST,
-                                ty::BoundRegion {
-                                    var: ty::BoundVar::from_usize(bound_vars.len() - 1),
-                                },
-                            ))
-                            .into()
-                        }
-                        }
-                        GenericParamDefKind::Const { .. } => {
-                            let bound_var = ty::BoundVariableKind::Const;
-                            bound_vars.push(bound_var);
-                            tcx.mk_const(ty::ConstS {
-                                ty: tcx.type_of(param.def_id),
-                                val: ty::ConstKind::Bound(
+                let subst_bound =
+                    if defs.count() == 0 {
+                        bound.subst(tcx, trait_predicate.trait_ref.substs)
+                    } else {
+                        let mut substs = smallvec::SmallVec::with_capacity(defs.count());
+                        substs.extend(trait_predicate.trait_ref.substs.iter());
+                        let mut bound_vars: smallvec::SmallVec<[ty::BoundVariableKind; 8]> =
+                            smallvec::SmallVec::with_capacity(
+                                bound.kind().bound_vars().len() + defs.count(),
+                            );
+                        bound_vars.extend(bound.kind().bound_vars().into_iter());
+                        InternalSubsts::fill_single(&mut substs, defs, &mut |param, _| match param
+                            .kind
+                            GenericParamDefKind::Type { .. } => {
+                            GenericParamDefKind::Type { .. } => {
+                                let kind = ty::BoundTyKind::Param(param.name);
+                                let bound_var = ty::BoundVariableKind::Ty(kind);
+                                bound_vars.push(bound_var);
+                                tcx.mk_ty(ty::Bound(
                                     ty::INNERMOST,
-                                    ty::BoundVar::from_usize(bound_vars.len() - 1),
-                            })
-                            .into()
-                        }
-                    });
-                    });
-                    let bound_vars = tcx.mk_bound_variable_kinds(bound_vars.into_iter());
-                    let assoc_ty_substs = tcx.intern_substs(&substs);
+                                    ty::BoundTy {
+                                        var: ty::BoundVar::from_usize(bound_vars.len() - 1),
+                                    },
+                                ))
+                                .into()
+                            }
+                            }
+                            GenericParamDefKind::Lifetime => {
+                                let kind = ty::BoundRegionKind::BrNamed(param.def_id, param.name);
+                                let bound_var = ty::BoundVariableKind::Region(kind);
+                                bound_vars.push(bound_var);
+                                tcx.mk_region(ty::ReLateBound(
+                                    ty::INNERMOST,
+                                    ty::BoundRegion {
+                                        var: ty::BoundVar::from_usize(bound_vars.len() - 1),
+                                    },
+                                ))
+                                .into()
+                            }
+                            }
+                            GenericParamDefKind::Const { .. } => {
+                                let bound_var = ty::BoundVariableKind::Const;
+                                bound_vars.push(bound_var);
+                                tcx.mk_const(ty::ConstS {
+                                    ty: tcx.type_of(param.def_id),
+                                    val: ty::ConstKind::Bound(
+                                        ty::INNERMOST,
+                                        ty::BoundVar::from_usize(bound_vars.len() - 1),
+                                })
+                                .into()
+                            }
+                        });
+                        });
+                        let bound_vars = tcx.mk_bound_variable_kinds(bound_vars.into_iter());
+                        let assoc_ty_substs = tcx.intern_substs(&substs);
 
-                    let bound_vars = tcx.mk_bound_variable_kinds(bound_vars.into_iter());
-                    let bound = bound.kind().skip_binder().subst(tcx, assoc_ty_substs);
-                    tcx.mk_predicate(ty::Binder::bind_with_vars(
-                        bound,
-                        bound_vars,
-                };
-                };
+                        let bound_vars = tcx.mk_bound_variable_kinds(bound_vars.into_iter());
+                        let bound = bound.kind().skip_binder().subst(tcx, assoc_ty_substs);
+                        tcx.mk_predicate(ty::Binder::bind_with_vars(bound, bound_vars))
                 let normalized_bound = normalize_with_depth_to(
                     self,
                     obligation.param_env,
                     obligation.param_env,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs" "/checkout/compiler/rustc_trait_selection/src/traits/object_safety.rs" "/checkout/compiler/rustc_trait_selection/src/traits/relationships.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
