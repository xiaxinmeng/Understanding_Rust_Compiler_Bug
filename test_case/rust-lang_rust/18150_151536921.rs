
struct Foo { a: usize }
fn main() {
    let mut x = std::rc::Rc::new(Foo { a: 1 });
    x.a += 1;

    let mut y = &(Foo { a: 1 });
    y.a += 1;
}
