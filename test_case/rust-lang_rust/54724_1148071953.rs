
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

#[proc_macro]
pub fn inner_using_outer_declarations_via_fn(_input: TokenStream) -> TokenStream {
    //let do_something = Ident::new("do_something", Span::call_site()); // Enabling causes compile error
    let do_something = Ident::new("do_something", Span::def_site());
    quote!(
        println!("inner_using_outer_declarations_via_fn:+ a={}", a);
        fn #do_something(a: i32) -> i32 {
            a + 1
        }
        a = #do_something(a);
        println!("inner_using_outer_declarations_via_fn:- a={}", a);
    )
    .into()
}

#[proc_macro]
pub fn inner_using_outer_declarations_via_temp(_input: TokenStream) -> TokenStream {
    let temp = Ident::new("do_something", Span::call_site());
    quote!(
        println!("inner_using_outer_declarations_via_temp:+ a={}", a);
        let #temp = a + 1;
        a = #temp;
        println!("inner_using_outer_declarations_via_temp:- a={}", a);
    )
    .into()
}

#[proc_macro]
pub fn outer(_input: TokenStream) -> TokenStream {
    let q = quote! {
        let mut a = 10;
        inner_using_outer_declarations_via_fn!();
        inner_using_outer_declarations_via_temp!();
        inner_using_outer_declarations_via_fn!();
        inner_using_outer_declarations_via_temp!();
    };
    //println!("outer: q={:#?}", q);
    q.into()
}
