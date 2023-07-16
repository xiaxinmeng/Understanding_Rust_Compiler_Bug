 rust
struct Bar;

impl Tr for Bar {
    fn me<A>(&self, a: A) {}
}

fn main() {
    let b = Bar;
    let obj = &b as &Tr;

    render(obj);
}
