rust
#[proc_macro_derive(Encode, attributes(encde))]
pub fn encode_derive_wrapper(input: TokenStream1) -> TokenStream1 {
	let input = parse_macro_input!(input as DeriveInput);
	let expanded = encode::derive(input);
	expanded.into()
}
