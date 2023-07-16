 rust
enum MinMax<T> {
    // Empty iterator
    NoElements,
    // Iterator with one element, so the minimum and maximum are the same (and
    // without Clone we can't make two copies of it)
    OneElement(T)
    // More than one element in the iterator, so we can store them separated
    MinMax(T, T)
}
