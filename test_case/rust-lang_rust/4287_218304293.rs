 rust
enum E1<T> {
    V1(T),
    V2(Box<E1<E2<T>>>)
}
