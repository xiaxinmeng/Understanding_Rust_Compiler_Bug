rust
// src/lib.rs
#![feature(proc_macro_span)]

use proc_macro::TokenStream;

#[proc_macro]
pub fn no_hygiene(tokens: TokenStream) -> TokenStream {
    tokens.into_iter().map(|mut tree| {
        let mut span = tree.span();
        while let Some(parent) = span.parent() {
            span = parent;
        }
        tree.set_span(tree.span().resolved_at(span));
        tree
    }).collect()
}
