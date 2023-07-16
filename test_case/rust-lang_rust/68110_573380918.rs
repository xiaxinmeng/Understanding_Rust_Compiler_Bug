rust
fn foo() -> impl Ord {
    if 0 == 1 { return 0 }
    else if 0 == 2 { return 1 }
    else { return 2 }
}
