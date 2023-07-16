Rust
struct X;
impl X {
    fn mutate(&mut self) {}
}

pub fn foo(x: bool) {
    let mut term = X;
    let ref_term = if x {
        &mut term
    } else {
        &X
    };
    ref_term.mutate();
}
