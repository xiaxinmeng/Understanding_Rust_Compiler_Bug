rust
fn parse_knobs(mut input: syn::ItemFn, is_test: bool, config: FinalConfig) -> TokenStream {
    input.sig.asyncness = None;
    ...
