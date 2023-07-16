rust
#[proc_macro]
pub fn add_mul(input: TokenStream) -> TokenStream {
    let mul_2 = vec![
        TokenTree::from(Punct::new('*', Spacing::Alone)),
        TokenTree::from(Literal::u8_unsuffixed(2)),
    ];
    input.into_iter().chain(mul_2.into_iter()).collect()
}
