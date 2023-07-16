rust
struct List<T> {
    value: Rc<T>,
    next: Option<Arc<List<T>>>
}
