 rust
// synext.rs
#![feature(plugin_registrar, quote, rustc_private)]
#![crate_name = "synext"]
#![crate_type = "dylib"]

extern crate syntax;
extern crate rustc_plugin;
use rustc_plugin::Registry;

use syntax::ext::base::{MacResult, MacEager};
use syntax::tokenstream;
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;

#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_macro("test", expand);
}

fn expand(cx: &mut ExtCtxt, _sp: Span,
          _tts: &[tokenstream::TokenTree]) -> Box<MacResult + 'static> {
    MacEager::expr(quote_expr!(cx,
                               std::io::stdout().write("one");
                               std::io::stdout().write("two");
    ))
}
