
struct A;
trait B { fn next(&self) { println!("trait"); } }

impl B for A {}
impl A {
    fn next(&self) { println!("inherent"); }
}

fn main() {
    let a = A;
    a.next()
}
