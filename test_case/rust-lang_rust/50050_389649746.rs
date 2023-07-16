rust
#[proc_macro_attribute]
pub fn demo(_args: TokenStream, _input: TokenStream) -> TokenStream {
    let ident = Ident::from("x");
    let expr = create(&ident);
    let tokens: Tokens = quote! {
        macro do_f($id:ident) {
            $id + 1
        }

        fn f(#ident: usize) -> usize {
            #expr
        }
    };

    tokens.into()
}
