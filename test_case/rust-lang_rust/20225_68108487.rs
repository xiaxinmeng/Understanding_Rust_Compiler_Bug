 rust
struct Test;
impl<T> Fn(T) for Test {
    extern "rust-call" fn call(&self, _args: (T,)) {}
}
