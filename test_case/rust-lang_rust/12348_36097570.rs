 rust
struct Cons<T> {
    head: T,
    tail: List<T>
}
type List<T> = Option<~Cons<T>>;
