rust
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn expand(_: TokenStream) -> TokenStream {
    quote! {
        output_mut(|o| o.copied_text = "".into());
        output_mut(|o| o.copied_text = format!("{:?}", self.tile_db));
    }
    .into()
}
