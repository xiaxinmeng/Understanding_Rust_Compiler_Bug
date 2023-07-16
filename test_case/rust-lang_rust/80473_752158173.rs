rust
#[derive(Debug, Hash)]
pub struct Struct {
    pub ident: String,
    pub args: Arguments,
    pub fields: Vec<Field>,
}
