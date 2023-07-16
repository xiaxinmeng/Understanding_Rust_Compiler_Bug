rust
token::Literal(token::Integer(name), suf) => {
    let span = self.span;
    self.bump();
    // Affix the suffix to the integer. This means that the suffix will
    // be treated as part of the field name and will not simply be ignored.
    let name = name.as_str().to_string() + suf.map_or("", |s| s.as_str().get());
    let field = ExprKind::Field(e, Ident::new(Symbol::intern(&name), span));
    e = self.mk_expr(lo.to(span), field, ThinVec::new());
}
