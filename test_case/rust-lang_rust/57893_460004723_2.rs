
// If the following goal has a non-empty set of solutions, reject the impl.
exists<T: type, 'a: lifetime, U: type, AT: list of auto-traits> [
    Unify(dyn Object<'a, Output = U> + AT, SomeType<T>),
    dyn Object<'a, Output = U> + AT: WC
}
