rust
use syntax::parse::{ParseSess, filemap_to_tts};
use syntax::tokenstream::TokenStream;


    let source_str = quote!(foo bar).as_str();
    let sess = ParseSess::new();
    let filemap =
        sess.codemap().new_filemap("<procmacro_lex>".to_string(), None, source_str.to_owned());
    let my_ts: TokenStream = filemap_to_tts(&sess, filemap).into_iter().collect();
