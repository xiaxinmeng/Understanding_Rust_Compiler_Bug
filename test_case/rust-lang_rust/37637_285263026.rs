rust
#[proc_macro]
pub fn print_input(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    println!("{}", source);
    input
}
