rust
    let item = parse_macro_input!(item as syn::ItemFn);
    let syn::ItemFn { attrs, sig, vis: _, block } = item;
...
    let ret_type = sig.output;
    let inputs = sig.inputs;
    let span = sig.ident.span();
    let ident = sig.ident;
    let output = quote_spanned! {span=>
        fn #ident () #ret_type {
            async fn func(#inputs) #ret_type {      <-- These braces are redundant, and
                #block
            }                        <-- create an inconsistent span map of the function body
            ...
        }
    }
