
trait Tr { fn bar(&'static self); }
impl Tr for usize { fn bar(&'static self) {} }
const FOO: usize = 1;
fn ex1() {
    (FOO + 1).bar()
}

fn ex2() {
    let _x: &'static _ = &(*&5 + 1);
}

fn main() {
    ex1();
    ex2();
}
