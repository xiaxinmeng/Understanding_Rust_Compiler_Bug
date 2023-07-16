 rust
loop {
    let new = f(current);
    let old = foo.compare_exchange(current, new, ord1, ord2);
    if old == current {
        break
    }
    current = old;
}
