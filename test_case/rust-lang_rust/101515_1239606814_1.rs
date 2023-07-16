rust
 fn parse_let_expr(&mut self) -> PResult<'a, P<Expr>> {
        ...

        self.expect(&token::Eq)?;
        
        ...
        Ok(self.mk_expr(span, ExprKind::Let(pat, expr, span)))
    }
