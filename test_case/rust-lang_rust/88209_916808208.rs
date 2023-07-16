rust
            if p.eat_keyword(kw::Underscore) {
                let err = ecx.struct_span_err(p.token.span, "_ cannot be used for input operands");
                return Err(err);
            }
            let expr = p.parse_expr()?;
