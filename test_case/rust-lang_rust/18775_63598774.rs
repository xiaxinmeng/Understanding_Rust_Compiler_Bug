 rust
#![feature(quote)]

extern crate syntax;

use syntax::print::pprust;
use syntax::codemap::DUMMY_SP;
use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;

#[allow(unused_imports)]
fn main() {
    let var_name = ast::Ident::new(
            token::intern("my_var")
    );
    let collection = ast::Ident::new(
            token::intern("my_collection")
    );
    with_fake_extctxt(|cx| {
        let field_set = quote_tokens!(cx,
            $var_name: self.$collection.pop(id),
        );
        println!("{}", pprust::tts_to_string(field_set.deref()));
    });
}

fn with_fake_extctxt<T>(f: |&syntax::ext::base::ExtCtxt| -> T) -> T {
    let ps = syntax::parse::new_parse_sess();

    let mut cx = syntax::ext::base::ExtCtxt::new(&ps, Vec::new(), syntax::ext::expand::ExpansionConfig {
        deriving_hash_type_parameter: false,
        crate_name: from_str("test").unwrap(),
        enable_quotes: true,
        recursion_limit: 100
    });

    cx.bt_push(syntax::codemap::ExpnInfo{
        call_site: DUMMY_SP,
        callee: syntax::codemap::NameAndSpan {
            name: "test".to_string(),
            format: syntax::codemap::MacroBang,
            span: None,
        }
    });

    f(&cx)
}
