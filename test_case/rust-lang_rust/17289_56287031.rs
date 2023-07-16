
fn bam(cx: &mut ExtCtxt, _: Span, tts: &[TokenTree]) -> Box<MacResult+'static> {
    // Parse an expression and emit it unchanged.
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), Vec::from_slice(tts));

    let interner = get_ident_interner();
    print!("{}\n", interner.len());
    for i in range(0, 56) {
        let n = Name(i);
        print!("{}\n", interner.get(n));
    }

    let expr = parser.parse_expr();
    MacExpr::new(expr)
}
