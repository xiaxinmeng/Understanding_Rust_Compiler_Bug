rust
#[proc_macro_derive(Serialize, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream;

#[proc_macro_attribute(attributes(serde))]
pub fn derive_serialize_as_an_attribute(args: TokenStream, input: TokenStream) -> TokenStream;
