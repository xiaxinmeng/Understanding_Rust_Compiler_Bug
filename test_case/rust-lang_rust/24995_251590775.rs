 rust
#![feature(rustc_private)]
extern crate syntax;

use syntax::ast;
use syntax::codemap::{DUMMY_SP, respan};
use syntax::parse::token;
use syntax::print::pprust;
use syntax::ptr::P;
use syntax::tokenstream;

fn main() {
    let item = P(ast::Item {
            ident: token::gensym_ident("DUMMY"),
            id: ast::DUMMY_NODE_ID,
            attrs: vec![],
            node: ast::ItemKind::Mac(
                respan(
                    DUMMY_SP,
                    ast::Mac_ {
                        path: ast::Path{ span: DUMMY_SP, global: false, 
                                segments: vec![ast::PathSegment{
                                    identifier: token::gensym_ident("my_macro"),
                                    parameters: ast::PathParameters::none()
                                  }]
                            },
                        tts: vec![tokenstream::TokenTree::Token(
                            DUMMY_SP,
                            token::Token::Literal(
                                token::Lit::Str_(
                                    token::gensym("abc")
                                        ), None))]
                        }
                    )
            ),
            vis: ast::Visibility::Inherited,
            span: DUMMY_SP,
        });
    println!("{}", pprust::item_to_string(&item));
    }
