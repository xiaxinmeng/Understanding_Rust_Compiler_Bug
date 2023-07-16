rust
                    // Carrier::translate(<expr>)
                    let discr = {
                        // expand <expr>
                        let sub_expr = self.lower_expr(sub_expr);

                        let path = &["ops", "Carrier", "translate"];
                        let path = P(self.expr_std_path(unstable_span, path, ThinVec::new()));
                        P(self.expr_call(e.span, path, hir_vec![sub_expr]))
                    };
