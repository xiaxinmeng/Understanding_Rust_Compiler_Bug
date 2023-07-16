rust
fn id<'a>(f: impl FnOnce() -> &'a mut Vec<()>) -> &'a mut Vec<()> {
    f()
}

fn foo() {
    let mut v: Vec<()> = Vec::new();
    id(|| &mut v).push(());
    assert_eq!(v.len(), 1);
}
