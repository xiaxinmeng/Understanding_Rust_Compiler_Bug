rust
// `method_call()` incorrectly resolves at call site, correctly has field span position
let trait_call = quote_span!(field.span, #field.method_call());
return quote!(#trait_call);

// `method_call()` correctly resolves at def site, incorrectly has def site span position
return quote!(#field.method_call());
