rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
