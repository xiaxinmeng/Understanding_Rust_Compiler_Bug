 rust
...
                } else {
                    // A nonterminal that matches or not
                    let namep = match p.token { token::Ident(_, p) => p, _ => token::Plain };
                    let name = p.parse_ident();
                    if p.token == token::Colon && p.look_ahead(1, |t| t.is_ident()) {
                        p.bump();
                        let kindp = match p.token { token::Ident(_, p) => p, _ => token::Plain };
                        let nt_kind = p.parse_ident();
                        let m = TtToken(sp, MatchNt(name, nt_kind, namep, kindp));
                        m
                    } else {
                        TtToken(sp, SubstNt(name, namep))
                    }
                }
