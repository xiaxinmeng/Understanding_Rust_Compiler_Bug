rust
macro m($($input:tt)*) {
    tokens ... $($input)* ...
}
// is equivalent to
#[proc_macro]
fn m(input: TokenStream) -> TokenStream {
    quote! { tokens ... $input ... }
}
