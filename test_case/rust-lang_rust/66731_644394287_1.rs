rust
        let expr = self.lower_expr(expr);
        let span = self.mark_span_with_reason(
            DesugaringKind::Await(expr.hir_id), await_span, None);
        let gen_future_span = self.mark_span_with_reason(
            DesugaringKind::Await(expr.hir_id),
            await_span,
            self.allow_gen_future.clone(),
        );
