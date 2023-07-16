
                if let ty::Ref(_, ty, mutability) = trait_ref.self_ty().skip_binder().kind() {
                    if self.tcx.erase_regions(*ty)
                        == self.tcx.erase_regions(candidates[0].self_ty())
                    {
                        err.span_suggestion_verbose(
                            span_from_obligation_variant_covering_expr.shrink_to_lo(),
                            &format!(
                                "consider borrowing the expression to use `{}`",
                                candidates[0],
                            ),
                            format!("&{}" mutability.prefix_str()),
                            Applicability::MachineApplicable,
                        );
                    }
                }
