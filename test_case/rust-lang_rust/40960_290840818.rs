Rust
pub fn expand_quote_item<'cx>(cx: &mut ExtCtxt,
                              sp: Span,
                              tts: &[TokenTree])
                              -> Box<base::MacResult+'cx> {
    let expanded = expand_parse_call(cx, sp, "parse_item_panic", vec!(), tts);
    base::MacEager::expr(expanded)
}
