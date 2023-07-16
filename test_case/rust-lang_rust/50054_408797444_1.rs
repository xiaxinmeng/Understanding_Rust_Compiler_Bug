rust
#[proc_macro_derive(Trait)]
pub fn derive(input: TokenStream) -> Result<TokenStream, Diagnostic> { }
