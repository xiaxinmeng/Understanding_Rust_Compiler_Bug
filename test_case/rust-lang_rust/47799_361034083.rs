rust
pub type Visibility = Spanned<VisibilityKind>;

#[derive(Clone, PartialEq, Eq, RustcEncodable, RustcDecodable, Hash, Debug)]
pub enum VisibilityKind {
    Public,
    Crate(CrateSugar),
    Restricted { path: P<Path>, id: NodeId },
    Inherited,
}
