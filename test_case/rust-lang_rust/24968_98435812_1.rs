
pub fn parse_ident_or_self_type(&mut self) -> PResult<ast::Ident> {
        if self.is_self_type_ident() {
            self.expect_self_type_ident()
        } else {
            self.parse_ident()
        }
    }
