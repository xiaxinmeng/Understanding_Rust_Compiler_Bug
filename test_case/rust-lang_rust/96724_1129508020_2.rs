rust
    self.check_keyword(kw::Extern)
        && self.look_ahead(1, |t| t.can_begin_literal_maybe_minus())
        && self.look_ahead(2, |t| t.is_keyword(kw::Fn))
