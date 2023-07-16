
// If the following goal has a non-empty set of solutions, reject the impl.
exists<T: type, 'a('x): lifetime -> lifetime, U('x): lifetime -> type, AT: list of auto-traits> [
    Unify(dyn for<'s> Object<'a('s), Output = U('s)> + AT, SomeType<T>),
    dyn for<'s> Object<'a('s), Output = U('s)> + AT: WC
}
