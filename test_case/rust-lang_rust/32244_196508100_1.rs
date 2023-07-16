 rust
loop {
    let new = f(current);
    match foo.compare_exchange(current, new, ord1, ord2) {
        Ok(..) => break,
        Err(old) => current = old,
    }
}
