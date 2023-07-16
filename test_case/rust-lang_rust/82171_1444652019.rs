rust
fn lifetime_issue() {
    let v = Vec::new();
    let _foo = foo(&v);
    drop(v); // do something with v
}

fn foo<'output, 'input_item, 'input_iter>(_: impl IntoIterator<Item = &'input_item String> + 'input_iter) -> impl Iterator<Item = String> + 'output
{
    std::iter::empty()
}
