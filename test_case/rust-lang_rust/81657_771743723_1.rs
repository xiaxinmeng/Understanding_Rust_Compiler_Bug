rust
match expr {
    Foo { x: Expr::Array(e), y: SomethingElse::Foo } => {...}
    ...
    Foo { x: #[non_exhaustive_fallback] _, y: SomethingElse::Foo } => { /* some sane fallback */ }
    Foo { x: _, y: _ } => { /* not a fallback */ }
}
