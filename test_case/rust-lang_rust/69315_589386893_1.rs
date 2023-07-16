rust
extern crate proc_macro;

use proc_macro::*;

#[proc_macro_attribute]
pub fn the_macro(_attr: TokenStream, body: TokenStream) -> TokenStream {
    let g = match body.clone().into_iter().nth(2) {
        Some(TokenTree::Group(g)) => g,
        _ => panic!("unexpected input"),
    };
    let tokens = g.stream().into_iter().collect::<Vec<_>>();
    assert_eq!(tokens[0].to_string(), "pub");
    return body;
}
