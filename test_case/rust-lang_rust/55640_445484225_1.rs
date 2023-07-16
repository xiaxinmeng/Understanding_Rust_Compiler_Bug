rust
            Ident(ident, is_raw) => tt!(Ident {
                sym: ident.name,
                is_raw
            }),
