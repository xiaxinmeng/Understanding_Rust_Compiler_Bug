rust
#[proc_macro_attribute]
pub fn attribute(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let file : Crate = parse( input ).unwrap();
    quote!( #file ).into()
}
