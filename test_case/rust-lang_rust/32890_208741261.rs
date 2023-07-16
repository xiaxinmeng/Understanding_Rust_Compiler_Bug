 rust
enum AssocItemLink<'a> {
    Anchor,
    GotoSource(DefId, &'a HashSet<String>),
}
