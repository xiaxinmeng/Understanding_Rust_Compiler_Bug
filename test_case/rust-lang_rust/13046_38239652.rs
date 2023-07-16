 rs
trait A {
    fn something() { println!("Hi"); }
}

trait B : A {
    fn something_else() { println!("Bye!"); }
}

struct Something;

impl B for Something {}
