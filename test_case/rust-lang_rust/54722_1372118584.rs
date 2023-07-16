rust
fn main() {
    let expr: syn::Expr = syn::parse_str("2 + 2").unwrap();
    let quoted = quote::quote! {
        let foo = &#expr;
    };
    println!("{quoted}"); // & 2 + 2
}
