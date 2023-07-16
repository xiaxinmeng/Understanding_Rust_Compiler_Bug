Rust
fn foo(x: &mut u32) {
    // both `s` and `t` are live for the generator's lifetime, but within
    // the generator they have distinct lifetimes.
    move || {
        {
            let s = &mut *x;
            yield;
            *s += 1;
        }

        let t = &mut *x;
        yield;
        *t += 1;
    };
}
