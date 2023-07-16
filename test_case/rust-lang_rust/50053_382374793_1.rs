rust 
#[proc_macro_attribute]
pub fn my_attr(attr: TokenStream, input: TokenStream, kind: SyntaxNodeKind) -> TokenStream {}
