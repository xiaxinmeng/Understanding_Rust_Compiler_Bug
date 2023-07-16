 rust
union U {
    i: i32,
    f: f32,
}

struct S {
    d: u32,
    u: U,
}

fn f(s: S) {
    unsafe {
        match s {
            S { d: 0, u: U { i: 42 } } => { ... }
        }
    }
}
