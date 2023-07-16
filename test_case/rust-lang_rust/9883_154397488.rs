 rust
struct Two<'a> {
    f: &'a i32,
    g: &'a i32,
}

fn lazy_lft() {
    let (mut t, f, g) : (Two, i32, i32);
    f = 42;
    t = Two { f: &f, g: &f };
    *t.f; // The lifetime definitely is already active here
    g = 23; // And g is definitely not yet borrowed.
    t.g = &g; // But now I can borrow g at the *old* lifetime
}
