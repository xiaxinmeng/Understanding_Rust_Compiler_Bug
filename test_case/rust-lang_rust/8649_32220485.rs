 rust
for x in xs.iter() {
    if cond(x) {
        xs.next(); // skip the next value
    }
    foo(x);
}
