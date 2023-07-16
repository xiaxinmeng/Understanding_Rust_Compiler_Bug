rust
pub(super) fn render_macro_matcher(matcher: &TokenTree) -> String {
    match matcher {
        TokenTree::Token(tok) => rustc_ast_pretty::pprust::token_to_string(tok),
        TokenTree::Delimited(_span, delim_tok, stream) => {
            let open_tok = Token::new(token::OpenDelim(*delim_tok), DUMMY_SP);
            let close_tok = Token::new(token::CloseDelim(*delim_tok), DUMMY_SP);
            format!(
                "{}{}{}",
                rustc_ast_pretty::pprust::token_to_string(&open_tok),
                stream.trees().map(|tt| render_macro_matcher(&tt)).collect::<String>(),
                rustc_ast_pretty::pprust::token_to_string(&close_tok)
            )
        }
    }
}
