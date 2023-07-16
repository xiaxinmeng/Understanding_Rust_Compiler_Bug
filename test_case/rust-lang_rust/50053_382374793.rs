rust
pub struct AttributeInput {
    // assume we'd use getters instead
    pub tokens: TokenStream,
    pub node_kind: SyntaxNodeKind, 
}

#[proc_macro_attribute]
pub fn my_attr(attr: TokenStream, input: AttributeInput) -> TokenStream {}
