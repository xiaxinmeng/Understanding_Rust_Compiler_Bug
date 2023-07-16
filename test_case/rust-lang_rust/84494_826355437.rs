rust
#[derive(Clone, Debug, Default)]
crate struct Attributes {
    crate doc_strings: Vec<DocFragment>,
    crate other_attrs: Vec<ast::Attribute>,
}
