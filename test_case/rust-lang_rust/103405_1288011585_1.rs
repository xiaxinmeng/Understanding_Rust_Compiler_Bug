rust
            // recovery from `if b && if let Some(x)`
            if op == AssocOp::LAnd
                && self.token.is_keyword(kw::If)
                && self.look_ahead(1, |t| t.is_bool_lit())
                || (self.look_ahead(1, |t| t.is_keyword(kw::Let))
                    && self.look_ahead(2, |t| {
                        t.is_ident_named(sym::Some) || t.is_ident_named(sym::None)
                    }))
            {
                // eat the `if`
                self.bump();
                self.sess.emit_err(UnexpectedIfWithIf(
                    self.prev_token.span.to(self.token.span.shrink_to_lo()),
                ));
            }
