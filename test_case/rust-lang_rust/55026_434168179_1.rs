rust
let gen = quote! {
        // #ast
        impl Countable for #name {
            fn count_fields(&self) -> usize {
                #n
            }
        }
    };

