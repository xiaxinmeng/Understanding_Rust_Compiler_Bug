text
// If the following goal has a non-empty set of solutions, reject the impl.
exists<T> {
    exists<U> {
        Unify(dyn Object<Output = U>, SomeType<T>),
        dyn Object<Output = U>: WC
    }
}
