 rust
#![cfg_attr(test, feature(stmt_expr_attributes))]

#[test]
fn test() {
    let _ = #[attr] (); //< Currently, this is a gated feature error (on non-test builds).
}
