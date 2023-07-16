
use std::cell::RefCell;

struct A;

impl A {
    fn id<'a>(&'a mut self) -> &'a mut A { self }

    fn bar(&mut self) {
        println!("oh no!");
    }
}

fn foo(f: ||) { f() }

fn main() {
    let mut v1 = A;
    let mut v2 = A;
    let c = RefCell::new(&mut v1);
    let d = RefCell::new(&mut v2);
    let mut r = d.borrow_mut();
    let mut p: &mut &mut A = r.get();
    c.set(&mut **p);
    p.bar(); //~ ERROR cannot borrow
}
