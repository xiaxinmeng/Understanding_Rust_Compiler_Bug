rust
#[proc_macro]
pub fn noop(input: TokenStream) -> TokenStream {
    input
}
