rust
#[proc_macro_derive(A)]
fn derive(input: TokenStream) -> TokenStream {
    let input = input.expand_macros();

    // --- OR ---

    let input = match input.try_expand_macros() {
        Ok(input) => input,
        Err(err) => return MacroContext::pending(err);
    };
}
