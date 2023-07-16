rust
#[proc_macro_derive(Foo)]
pub fn derive_foo(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input).unwrap();
    let ty = &ast.ident;
    let ty_name = ty.to_string();
    let in_self = env::var("CARGO_PKG_NAME").unwrap() == "foo";
    let trait_path = if in_self {
        Path {
            leading_colon: None,
            segments: ["crate", "Foo"].iter()
                .map(|&s| Ident::new(s, Span::call_site()))
                .map(PathSegment::from)
                .collect(),
        }
    } else {
        Path {
            leading_colon: Some(Default::default()),
            segments: ["foo", "Foo"].iter()
                .map(|&s| Ident::new(s, Span::call_site()))
                .map(PathSegment::from)
                .collect(),
        }
    };
    let tokens = quote! {
        impl #trait_path for #ty {
            fn foo(&self) -> String {
                format!("I'm a {}", #ty_name)
            }
        }
    };
    tokens.into()
}
