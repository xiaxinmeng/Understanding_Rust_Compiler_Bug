rust
fn bar<T: Foo<1>>() {
    println!("{}", T::FOO);
}
