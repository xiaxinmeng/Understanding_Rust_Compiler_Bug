rust
        quote_spanned!(name.span() =>
            mod #name {
                #[allow(unused_imports)]
                use super::*;

