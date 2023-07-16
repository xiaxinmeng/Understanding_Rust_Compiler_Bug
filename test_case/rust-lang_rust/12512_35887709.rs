 rust
pub struct Ident {
    name: Name,
    ctxt: SyntaxContext,
    kind: IdentKind
}

pub enum IdentKind {
    PlainIdent,
    LifetimeIdent
}
