rust
#[proc_macro_derive(JSTraceable)]
pub fn expand_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    // ...

    let style = synstructure::BindStyle::Ref.into();
    let match_body = synstructure::each_field(&mut type_, &style, |binding| {
        Some(quote! { #binding.trace(tracer); })
    });

    // ...
