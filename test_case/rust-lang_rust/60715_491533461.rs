rust
struct S {
    i: usize,
    //j: std::cell::Cell<usize>,
    //k: Box<dyn std::error::Error>,
}

fn f(s: &S) {
    let _ = std::panic::catch_unwind(|| {
        let _s = s;
    });
}

fn main() {}
