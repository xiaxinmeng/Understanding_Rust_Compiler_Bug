rust
struct Test<T>(T);

impl<T> Test<T> {
    const fn const_drop(self) {
    }
}
