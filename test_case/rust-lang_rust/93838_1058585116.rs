rust
fn foo<T>() {
    const _: () = {
        panic!("Const eval!");
    };
}
