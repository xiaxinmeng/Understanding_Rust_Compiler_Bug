
            LitKind::ByteStr(ref bytes) => {
                let string = bytes
                    .iter()
                    .cloned()
                    .flat_map(ascii::escape_default)
                    .map(Into::<char>::into)
                    .collect::<String>();
                (token::ByteStr, Symbol::intern(&string), None)
            }
