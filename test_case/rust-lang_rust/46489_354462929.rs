rust
#![feature(proc_macro)]
#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::{quote, TokenStream, TokenTree, TokenTreeIter, TokenNode, Delimiter};

fn parse(iter: &mut TokenTreeIter) -> TokenStream {
    iter.next();
    iter.next();
    if let Some(TokenTree{span: _, kind: TokenNode::Group(Delimiter::None, stream)}) = iter.next() {
        quote!($stream)
    } else {
        TokenStream::empty()
    }
}

#[proc_macro]
pub fn my_macro(stream: TokenStream) -> TokenStream {
    parse(&mut stream.into_iter())
}
