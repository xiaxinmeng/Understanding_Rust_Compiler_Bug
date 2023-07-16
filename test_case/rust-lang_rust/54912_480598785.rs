rust
let q = quote! {
    #[allow(non_upper_case_globals)]
    const #_const: () = {
        ...
    };
}
