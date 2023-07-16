 rust
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("expand_string_to_expr", expand_string_to_expr);
}

fn expand_string_to_expr(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    use syntax::print::pprust;

    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), Vec::from_slice(tts));
    let arg = cx.expand_expr(parser.parse_expr());
    let expr_string = match arg.node {
        ast::ExprLit(spanned) => {
            match spanned.node {
                ast::LitStr(ref s, _) => s.to_string(),
                _ => {
                    cx.span_err(sp, format!(
                            "expected string literal but got `{}`",
                            pprust::lit_to_string(&*spanned)).as_slice());
                    return DummyResult::expr(sp)
                }
            }
        }
        _ => {
            cx.span_err(sp, format!(
                    "expected string literal but got `{}`",
                    pprust::expr_to_string(&*arg)).as_slice());
            return DummyResult::expr(sp)
        }
    };
    if !parser.eat(&token::EOF) {
        cx.span_err(parser.span, "only one string literal allowed");
        return DummyResult::expr(sp);
    }

    let mut p = parse::new_parser_from_source_str(cx.parse_sess(), cx.cfg(), "string_expr".to_string(), expr_string);
    return box ParserAnyMacro{
        parser: std::cell::RefCell::new(p),
    } as Box<MacResult>
}
