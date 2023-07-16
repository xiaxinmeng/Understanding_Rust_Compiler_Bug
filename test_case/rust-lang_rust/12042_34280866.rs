 rust
struct Foo {
    a: int
}

struct Bar<'a> {
    a: &'a Foo,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropping foo");
    }
}

#[unsafe_destructor]
impl<'a> Drop for Bar<'a> {
    fn drop(&mut self) {
        println!("dropping bar {}", self.a.a);
    }
}

fn main() {
    let mut a = ~[];
    let b = Foo { a: 2 };
    a.push(Bar { a: &b });
}
