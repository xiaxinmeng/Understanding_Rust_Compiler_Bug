 rust
#![feature(quote)]
#![feature(macro_rules)]
extern crate syntax;
extern crate rustc;

use syntax::ast;
use syntax::codemap;
use syntax::parse;
use syntax::parse::token;
use syntax::print::pprust;

fn main() {
    let ctxt = Ctxt { parse: parse::new_parse_sess() };
    let e = quote_expr!(&ctxt, x + y);
    println!("expr: {}", pprust::expr_to_string(&*e));
    let p = quote_pat!(&ctxt,  x + y);
    println!("pat:  {}", pprust::pat_to_string(&*p));
}

struct Ctxt {
    parse: parse::ParseSess
}

impl Ctxt {
    fn parse_sess(&self)         -> &syntax::parse::ParseSess { &self.parse }
    fn cfg(&self)                -> ast::CrateConfig          { vec![] }
    fn call_site(&self)          -> codemap::Span             { codemap::DUMMY_SP }
    fn ident_of(&self, st: &str) -> ast::Ident                { token::str_to_ident(st) }
}
