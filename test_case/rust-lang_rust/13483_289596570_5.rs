rust
 pub fn parse_if_expr(&mut self, attrs: ThinVec<Attribute>) -> PResult<'a, P<Expr>> {
        // XXX I am adding this:
        let if_span = self.prev_span;

        if self.check_keyword(keywords::Let) {
            return self.parse_if_let_expr(attrs);
        }
        let lo = self.prev_span.lo;
        let cond = self.parse_expr_res(Restrictions::RESTRICTION_NO_STRUCT_LITERAL, None)?;

        // XXX adding this code too:
        if self.token.is_keyword(keywords::Else) {
            // Try to give a nice error for the case where user forgot an expression.
            // See #13483 for details.
            return Err(self.span_fatal(if_span, &format!("`if` without any condition expression")));
        }

        let thn = self.parse_block()?;
        /* ... */        
}
