rust

 let gen = quote! {
        impl #impl_generics ::Countable for #name #ty_generics #where_clause {
            fn count_fields(&self) -> usize {
                #n
            }
        }
    };
