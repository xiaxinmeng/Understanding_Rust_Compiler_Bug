rust
// in a world where `struct HashSet<T: Hash+Eq>`...
fn foo<T>(s: &mut HashSet<T>, a: T, b: T) -> bool {
    let r = a == b; // not allowed, because _this_ method doesn't know T: PartialEq
    s.insert(a); // allowed, because the method's `Self` type is HashSet, where we always know Hash+eq
    r
}
