
fn this_is_evil<T, Iter: Iterator<T>>(iter: Iter) -> _: Iterator<T> {
    iter
}

fn this_is_bad() -> ~Iterator<int>{
    // lifetime is this function
    let a = &[1, 2, 3];

    // the return type is an unknown type that impls `Iterator`,
    // so this is accepted, and now the trait object we return
    // points into invalid memory
    ~this_is_evil(a.iter().map(|i| *i))
    // at entry to `this_is_evil` the lifetime information is present,
    // but when it comes back out it is lost
}
