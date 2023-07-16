 rust
struct A { x: &'self mut int }
struct B { x: A<'self> }

impl Drop for A<'self> {
    fn finalize(&self) {
        *(self.x) -= 1; error!("A");
    }
}

fn main() {
    let mut y = 3;
    let bp = ~B { x: A { x: &mut y } };
    let ~B { x: _a } = bp;
    fail_unless!(y == 2);
}
