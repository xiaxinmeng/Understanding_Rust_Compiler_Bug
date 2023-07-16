rust
#[proc_macro_attribute]
pub fn attribute(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let file : Crate = parse_str( &input.to_string() ).unwrap();
    quote!( #file ).into()
}
