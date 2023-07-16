rust
use std::rc::Rc;
use std::mem;

pub enum InnerFoo {
    Branch(Rc<InnerFoo>),
    Leaf(usize),
}

pub struct Foo(InnerFoo);

impl Drop for Foo {
    fn drop(&mut self) {
        let mut current = InnerFoo::Leaf(0);
        mem::swap(&mut self.0, &mut current);
        loop {
            if let InnerFoo::Branch(mut next) = current {
                current = InnerFoo::Leaf(0);
                mem::swap(&mut current, Rc::get_mut(&mut next).unwrap());
            } else {
                break;
            }
        }
    }
}

pub fn deeper(mut p : Foo) -> Foo {
    let mut current = InnerFoo::Leaf(0);
    mem::swap(&mut p.0, &mut current);
    p.0 = InnerFoo::Branch(Rc::new(current));
    return p;
}

pub fn main() {
    let mut p = Foo(InnerFoo::Leaf(1));
    
    for _ in 0..100000 {
        p = deeper(p)
    }
}
