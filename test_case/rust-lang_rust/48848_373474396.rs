
#[derive(Clone, PartialEq)]
struct MyArc<
    #[derive_no_bound(Clone)]
    T,
>(Arc<T>);
