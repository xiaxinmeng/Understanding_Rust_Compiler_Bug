 rust
use std::rc::Rc;

struct Bar { x: int }

fn foo(p: &int) {
    println!("{}", p);
}

fn baz(p: &Bar) {
    println!("{}", p.x);
}

fn main() {
    let pi = box 10;
    let pb = Rc::new(Bar {x: 3});
    foo(pi);
    // this still doesn't auto borrow:
    //baz(pb);
    // although this works:
    println!("{}", pb.x);
}
