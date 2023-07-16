rust
// macro crate
#[proc_macro]
pub fn make_answer(ts: TokenStream) -> TokenStream {
    let krate = ts.parse_path(); // pseudo-code
    _ = ts.parse_semicolon();

    // ...
}
