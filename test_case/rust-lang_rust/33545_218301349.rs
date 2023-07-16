 rust
enum A<T> {
    A1(T),
    A2(Box<A<::std::marker::PhantomData<T>>>)
}

fn main() {
    A::A1(());
}
