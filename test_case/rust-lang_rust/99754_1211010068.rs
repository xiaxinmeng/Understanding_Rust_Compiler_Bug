rust
enum FieldDefKind {
    Named(Ident, P<Ty>),
    Unnamed(P<Ty>),
    Nested(StructOrUnion, Vec<FieldDef>)
}
