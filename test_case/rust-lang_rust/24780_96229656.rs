
fn foo() -> Vec<usize>> {
    //                ^
    // error: expected one of `(`, `+`, `::`, `<`, `>`, `where`, or `{`, found `>`
    Vec::new()
}
