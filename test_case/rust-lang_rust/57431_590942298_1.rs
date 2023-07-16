rust
struct X;
impl X {
    fn mutate(&mut self) {}
}

pub fn foo() {
    let mut term = X;
    let ref_term: &mut X = &mut term;
    ref_term.mutate();
}

fn main() { }
