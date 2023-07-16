Rust
match e.node {
    ExprMatch(..) => true,
    _ => span_bug!(e.span, "unexpected expr {:?}", e.node)
}
