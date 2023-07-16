 rust
macro_rules! m { () => { 2; } }
fn f() -> i32 {
    let _: i32 = m!(); // The semicolon after `2` is ignored, so this is typechecks.
    m!() // Before f639fc3, this invocation is considered to be in an expression position,
         // so the semicolon after the `2` is ignored and this typechecks.
         //
         // After f639fc3, the invocation is considered to be in a statement position, so
         // the semicolon is not ignored and it expands into a `StmtKind::Semi` statement,
         // which doesn't typecheck.
}
