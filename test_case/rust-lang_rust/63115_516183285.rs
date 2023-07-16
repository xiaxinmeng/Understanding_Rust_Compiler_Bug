rust
    fn is_pat_range_end_start(&self) -> bool {
        self.token.is_path_start() // e.g. `MY_CONST`;
            || self.token == token::Dot // e.g. `.5` for recovery;
            || self.token.can_begin_literal_or_bool() // e.g. `42`.
    }
