rust
pub struct Example;

trait A {
    fn do_a(&self);
}

trait B {
    fn do_b(&self);
}

trait Empty {}

impl <T> B for T where T: A + Empty
{
    fn do_b(&self) {
        self.do_a()
    }
}

impl A for Example {
    fn do_a(&self) {
        println!("A!");
    }
}

fn main() {
    let e = Example;
    e.do_b();
}
