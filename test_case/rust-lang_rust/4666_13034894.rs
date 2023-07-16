
struct A { a: int }

struct B { a: &A }

impl B {
    fn next1(&mut self) { }
    fn next2(self) -> B/&self { let B { a: a } = self; B { a: a } }
}

fn main() {
    let a = A { a: 0 };
    let mut b = B { a: &a };
    //b.next1(); b.next1();
    //for uint::range(0, 1) |_| { b.next1(); }
    //b = b.next2(); b = b.next2();
}
