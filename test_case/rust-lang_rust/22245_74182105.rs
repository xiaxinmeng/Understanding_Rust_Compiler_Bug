 rust
pub fn expand_quote_tokens<'cx>(cx: &'cx mut ExtCtxt,
                                sp: Span,
                                tts: &[ast::TokenTree])
                                -> Box<base::MacResult+'cx> {

    cx.sess.macro_features.gate(/* name of the feature */ "asm", /* name of the macro */ "asm");
