rust
        match tt {
            TokenTree::Token(token) => {
                let token_str = self.token_to_string_ext(&token, convert_dollar_crate);
                self.word(token_str);
                if let token::DocComment(..) = token.kind {
                    self.hardbreak()
                }
            }
            // ...
