Rust
#[proc_macro]
pub fn python(_input: TokenStream) -> TokenStream {
    let source_text = proc_macro::Span::call_site().source_text().unwrap();
    format!("{source_text:?}").parse().unwrap()
}
