rust
fn f<T>() {
    fn f() {
        std::mem::size_of::<T>(); // ERROR can't use generic parameters from outer function
    }
}
