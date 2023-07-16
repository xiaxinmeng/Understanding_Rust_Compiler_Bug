rust
fn foo<T: ?Sized + SomeTrait>() {
    T::static_method();
}

fn main() {
    foo::<SomeTrait>();
}
