rust
#[phase_1] // With `extern crate` disappearing, we may never introduce this...
extern crate foo; // (1) This declares a target dependency `foo`

#[proc_macro]
fn m(input: TokenStream) -> TokenStream {
    use foo; // This doesn't resolve
    quote! { // n.b. `quote` creates tokens with `Span::def_site()`.
        use foo; // This resolves to (1)
    }
}
