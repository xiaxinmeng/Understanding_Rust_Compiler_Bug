rust
    fn recover_nested_adt_item(&mut self, keyword: Symbol) -> PResult<'a, bool> {
        if (self.token.is_keyword(kw::Enum) ||
            self.token.is_keyword(kw::Struct) ||
            self.token.is_keyword(kw::Union))
            && self.look_ahead(1, |t| t.is_ident())
        {
