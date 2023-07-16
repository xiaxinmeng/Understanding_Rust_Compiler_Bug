rust
enum Node<T> where <RcFamily as PointerFamily>::Pointer<Node<T>>: Sized {
    Cons(T, <RcFamily as PointerFamily>::Pointer<Node<T>>),
}
