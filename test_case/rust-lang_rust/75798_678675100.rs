
extern crate proc_macro;

use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_derive(GraphQLQuery)]
pub fn hello_macro_derive(_input: TokenStream) -> TokenStream {
    TokenStream::from_str(&recursive_call(42).to_string()).unwrap()
}

fn recursive_call(n: i32) -> i32 {
    return recursive_call(n + 1);
}
