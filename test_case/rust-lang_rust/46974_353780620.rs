Rust
#![feature(nll)]

fn foo(s: &mut (i32,)) -> i32 {
    let t = &mut *s;
    let x = &t.0; // or {s}
    *s = (2,);
    *x    // 2, which is clearly incorrect
}
