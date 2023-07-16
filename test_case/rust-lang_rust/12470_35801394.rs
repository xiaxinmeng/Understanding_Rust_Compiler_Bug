 rust
trait X {
    fn get_i(&self) -> int;
}


struct B {
    i: int
}

impl X for B {
    fn get_i(&self) -> int {
        self.i
    }
}

impl Drop for B {
    fn drop(&mut self) {
        println!("drop");
    }
}

struct A<'r> {
    p: &'r X
}

fn make_a<'r>(p:&'r X) -> A<'r> {
    A{p:p}
}

fn make_make_a() -> A {
    let b: ~B = ~B {i:1};
    let bb: & B = b;
    make_a(bb)
}

fn main() {
    let a = make_make_a();
    println!("{}", a.p.get_i());
}
