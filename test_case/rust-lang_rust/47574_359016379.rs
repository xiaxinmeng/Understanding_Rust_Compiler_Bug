rust
#[derive(Clone, Copy, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
pub struct ParamTy {
    pub idx: u32,
    pub name: Name,
    pub span: Span,
}
