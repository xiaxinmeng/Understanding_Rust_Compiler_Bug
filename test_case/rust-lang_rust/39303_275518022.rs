rust
fn ident_can_begin_expr(ident: ast::Ident) -> bool {
    let ident_token: Token = Ident(ident);

    !ident_token.is_any_keyword() ||
    ident_token.is_path_segment_keyword() ||
    [
        keywords::Box.name(),
        keywords::Break.name(),
        keywords::Continue.name(),
        keywords::False.name(),
        keywords::For.name(),
        keywords::If.name(),
        keywords::Loop.name(),
        keywords::Match.name(),
        keywords::Move.name(),
        keywords::Return.name(),
        keywords::True.name(),
        keywords::Unsafe.name(),
        keywords::While.name(),
    ].contains(&ident.name)
}
