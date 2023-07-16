 Rust
    fn expr_field_access(&self, sp: Span, expr: P<ast::Expr>, ident: ast::Ident) -> P<ast::Expr> {
          let field_span = Span {
              lo: sp.lo - Pos::from_usize(ident.name.as_str().len()),
              hi: sp.hi,
              expn_id: sp.expn_id,
          };
          // ...
    }
