
struct Query<T> {
    inner: RefCell<QueryState<T>>,
}

enum QueryState<T> {
    Empty,
    Value(T),
    Error(SomeErrorType),
    Stolen,
}
