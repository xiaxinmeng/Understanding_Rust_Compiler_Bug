rust
struct Foo<T>(T);
impl<T> Foo<T> {
    const BAR: usize = 42;
}

fn muh<T>() {
    match 42 {
        Foo::<T>::BAR => {}
        _ => {}
    }
}
