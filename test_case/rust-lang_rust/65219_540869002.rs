rust
fn invert<T>(predicate: impl FnMut(T) -> bool) -> impl FnMut(T) -> bool {
    move |value| !predicate(value)
}
// I also tried FnMut(&T) -> bool and FnMut(&'a T) -> bool + 'a

// ...

right.extend(fused.filter(invert(predicate)));
