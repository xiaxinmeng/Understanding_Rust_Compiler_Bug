rust
impl<'a, 'b> MutVisitor for InvocationCollector<'a, 'b> {
    fn visit_expr(&mut self, expr: &mut P<ast::Expr>) {
        self.cfg.configure_expr(expr);
        visit_clobber(expr.deref_mut(), |mut expr| {
            self.cfg.configure_expr_kind(&mut expr.kind);

            // ignore derives so they remain unused
            let (attr, after_derive) = self.classify_nonitem(&mut expr);

            if attr.is_some() {
                // Collect the invoc regardless of whether or not attributes are permitted here
                // expansion will eat the attribute so it won't error later.
                attr.as_ref().map(|a| self.cfg.maybe_emit_expr_attr_err(a));

                // AstFragmentKind::Expr requires the macro to emit an expression.
                return self
                    .collect_attr(
                        attr,
                        vec![],
                        Annotatable::Expr(P(expr)),
                        AstFragmentKind::Expr,
                        after_derive,
                    )
                    .make_expr()
                    .into_inner();
            }

            if let ast::ExprKind::MacCall(mac) = expr.kind {
                self.check_attributes(&expr.attrs);
                self.collect_bang(mac, expr.span, AstFragmentKind::Expr).make_expr().into_inner()
            } else {
                noop_visit_expr(&mut expr, self);
                expr
            }
        });
    }
