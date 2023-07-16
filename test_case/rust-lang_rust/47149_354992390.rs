rust
let trait_call = quote_span!(field.span, #field.method_call());
return quote!(#trait_call);
