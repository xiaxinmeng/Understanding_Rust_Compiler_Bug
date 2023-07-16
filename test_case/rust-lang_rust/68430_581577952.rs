rust
pub fn async_sync_trait(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let mut item = parse_macro_input!(input as Item);
    // Adds some methods
    expand(&mut item, args.local);
    if args.local {
        TokenStream::from(quote! {
            #[async_trait::async_trait(?Send)]
            #item
        })
    } else {
        TokenStream::from(quote! {
            #[async_trait::async_trait]
            #item
        })
    }
}
