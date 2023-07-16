rust
match expr {
    Expr::Array(e) => {...}
    Expr::Assign(e) => {...}
    ...
    Expr::Yield(e) => {...}
    #[non_exhaustive_fallback] _ => { /* some sane fallback */ }
}
