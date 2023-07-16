rust

struct A {
    a1: Option<unsafe extern fn()>,
    a2: (),
}

impl A {
    fn new() -> Self {
        A {
            a1: None,
            a2: (),
        }
    }
}

fn main() {
    A::new();
}
