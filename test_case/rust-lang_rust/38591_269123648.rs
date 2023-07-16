rust
struct S<T> {
    t : T,
    s : Box<S<Option<fn(u : T)>>>
}
