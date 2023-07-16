rust
// lib.rs

#[proc_macro]
pub fn proc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let proc_macro::TokenTree::Group(group) = input.into_iter().nth(1).unwrap() {
        println!("proc macro: #[doc = {}]", group.stream().into_iter().nth(2).unwrap());
    }
    proc_macro::TokenStream::new()
}
