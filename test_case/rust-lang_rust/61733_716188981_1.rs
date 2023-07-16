rust
fn trailing_item() -> bool {
    { true }
    fn inner() {}
}

fn trailing_stmt() -> bool {
    { true }
    let a = 1;
}

fn trailing_smei() -> bool {
    { true }
    ;
}
