
            match (cm.lookup_line(self.span.lo()), cm.lookup_line(sp.lo())) {
                (Ok(ref a), Ok(ref b)) if a.line == b.line => {
                    // When the spans are in the same line, it means that the only content between
                    // them is whitespace, point at the found token in that case:
                    //
                    // X |     () => { syntax error };
                    //   |                    ^^^^^ expected one of 8 possible tokens here
                    //
                    // instead of having:
                    //
                    // X |     () => { syntax error };
                    //   |                   -^^^^^ unexpected token
                    //   |                   |
                    //   |                   expected one of 8 possible tokens here
                    err.span_label(self.span, label_exp);
                }
                _ if self.prev_span == syntax_pos::DUMMY_SP => {
                    // Account for macro context where the previous span might not be
                    // available to avoid incorrect output (#54841).
                    err.span_label(self.span, "unexpected token");
                }
                _ => {
                    err.span_label(sp, label_exp);
                    err.span_label(self.span, "unexpected token");
                }
            }
