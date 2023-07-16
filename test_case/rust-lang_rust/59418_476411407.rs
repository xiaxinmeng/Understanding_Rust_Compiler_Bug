
                    token::Literal(token::Integer(name), suffix) => {
                        let span = self.span;
                        self.bump();
                        let field = ExprKind::Field(e, Ident::new(name, span));
                        e = self.mk_expr(lo.to(span), field, ThinVec::new());
+                       if let Some(suffix) = suffix {
+                           let mut err = self.diagnostic().struct_span_err(
+                               span,
+                               "tuple index with a suffix is invalid",
+                           );
+                           err.span_label(span, format!("invalid suffix `{}`", suffix));
+                           err.emit();
+                       }
                    }
