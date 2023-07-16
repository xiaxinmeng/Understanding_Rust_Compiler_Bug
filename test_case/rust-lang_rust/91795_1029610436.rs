rust
// This struct is used during decoding and unifies both proper items and reexports
struct ModChild {
    pub ident: Ident,
    pub res: Res<!>,
    pub vis: ty::Visibility,
    pub span: Span,
    pub macro_rules: bool,
}

// This structure is used for encoding the reexport table, `macro_rules` is always false by definition
pub struct Reexport {
    pub ident: Ident,
    pub res: Res<!>,
    pub vis: ty::Visibility,
    pub span: Span,
}
