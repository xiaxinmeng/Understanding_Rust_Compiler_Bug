
// Skip trivial (whitespace & comments) tokens
loop {
    // ....
    match self.cook_lexer_token(token.kind, start) {
        Some(kind) => {
            let span = self.mk_sp(start, self.pos);
            return (spacing, Token::new(kind, span));
        }
        None => spacing = Spacing::Alone,
    }
}
