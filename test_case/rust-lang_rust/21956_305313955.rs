rust
struct A<'a> {
    s: &'a str,
}

impl<'a> A<'a> {
    fn perform_mut(&'a mut self) -> &'a str {
        self.s
    }
}

fn main() {
    let mut x = A { s: "A" };
    x.perform_mut();
    x.perform_mut();
}
