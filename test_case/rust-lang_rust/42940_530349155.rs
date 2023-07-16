rust
fn foo<T>(_: T) -> impl Iterator<Item = i32> + 'static {
    vec![2].into_iter()
}

fn bar() {
    let input = vec![1];
    let output0 = foo(&input); //output0 should have nothing to do with input
    let output1 = foo(input);
}
