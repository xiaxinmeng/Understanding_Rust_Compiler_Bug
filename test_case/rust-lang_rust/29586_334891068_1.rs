
        let require_comma = classify::expr_requires_semi_to_be_stmt(&expr)
            && self.token != token::CloseDelim(token::Brace);

        if require_comma {
            self.expect_one_of(&[token::Comma], &[token::CloseDelim(token::Brace)])?;
        } else {
            self.eat(&token::Comma);
        }
