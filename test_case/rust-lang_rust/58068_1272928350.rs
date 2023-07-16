rust
use std::rc::Rc;

enum Foo {
    Branch(Rc<Foo>),
    Leaf(usize),
}

fn deeper(p : Foo) -> Foo {
    Foo::Branch(Rc::new(p))
}

fn main() {
    let mut p = Foo::Leaf(1);
    
    for _ in 0..100000 {
        p = deeper(p)
    }
}
