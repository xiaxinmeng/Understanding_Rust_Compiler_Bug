rust
fn generic<T>() {}

fn foo<T>() {
    const _: () = generic::<T>();
}
