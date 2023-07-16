rust
            Ident(ident, false) => tt!(self::Ident::new(&ident.as_str(), Span(span))),
            Ident(ident, true) => tt!(self::Ident::new_raw(&ident.as_str(), Span(span))),
