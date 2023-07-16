
pub fn inner_using_outer_declarations_via_fn(_input: TokenStream) -> TokenStream {
    let do_something = Ident::new("do_something", Span::call_site()); // Enabling causes compile error
    //let do_something = Ident::new("do_something", Span::def_site());
