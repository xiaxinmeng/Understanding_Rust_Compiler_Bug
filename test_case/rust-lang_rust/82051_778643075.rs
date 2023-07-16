rust
    fn is_certainly_not_a_block(&self) -> bool {
        self.look_ahead(1, |t| t.is_ident())
            && (
                // `{ ident, ` cannot start a block.
                self.look_ahead(2, |t| t == &token::Comma)
                    || (self.look_ahead(2, |t| t == &token::Colon)
                        && (
                            // `{ ident: token, ` cannot start a block.
                            self.look_ahead(4, |t| t == &token::Comma) ||
                            // `{ ident: ` cannot start a block unless it's a type ascription `ident: Type`.
                            self.look_ahead(3, |t| !t.can_begin_type())
                        ))
            )
    }
