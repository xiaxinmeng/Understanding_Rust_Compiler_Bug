rust
// src/lib.rs

#![feature(proc_macro_span)]

use proc_macro::TokenStream;

#[proc_macro]
pub fn source_text(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let begin = iter.next().unwrap().span();
    let end = iter.next().unwrap().span().resolved_at(begin);
    let joined = begin.join(end).unwrap();
    let source_text = joined.source_text().unwrap();
    format!("{source_text:?}").parse().unwrap()
}
