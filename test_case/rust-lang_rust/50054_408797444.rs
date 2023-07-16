rust
type PResult<T> = ::std::result::Result<T, Diagnostic>;

fn real_derive(input: TokenStream) -> PResult<TokenStream> {
    // return early if an error occurs
    return Err(span.error("foo"));
}

#[proc_macro_derive(Trait)]
pub fn derive(input: TokenStream) -> TokenStream {
    real_derive(input).unwrap_or_else(|diag| {
        diag.emit();
        TokenStream::new()
    })
}
