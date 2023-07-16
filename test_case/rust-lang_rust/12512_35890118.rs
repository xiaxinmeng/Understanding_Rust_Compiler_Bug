 rust
pub enum IdentKind {
    PlainIdent,
    LifetimeIdent
}

mtwt_resolve(...) => mtwt_resolve(..., IdentKind)
new_rename(...) => new_rename(..., IdentLind)
new_mark(...) => new_mark(..., IdentKind)
