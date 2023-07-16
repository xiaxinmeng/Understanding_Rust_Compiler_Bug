rust
enum Resolution {
    Def(DefKind, DefId),  // replaces `Res::Def`
    Primitive { fragment: String },
}
