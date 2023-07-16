rust
 if self.prev_token.is_ident_named(sym::public) && self.token.can_begin_item() {
            err.span_suggestion_short(
                self.prev_token.span,
                "write `pub` instead of `public` to make the item public",
                "pub",
                appl,
            );
        }
