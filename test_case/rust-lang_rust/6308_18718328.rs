
fn bar<T: Sized>() { }
fn foo<T>() {
     bar::<~[T]>()
}
