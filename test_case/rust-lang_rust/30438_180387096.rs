 rust
trait Trait { type Out; fn method(&self) -> &Self::Out; }
struct Test<'a> { s: &'a String }

impl<'a> Trait for Test<'a> {
type Out = Test<'a>;
    fn method(&self) -> &Self::Out {
        &Test { s: &self.s }
    }
}

fn main() {
    let s = "Hello World".to_string();
    let test = Test { s: &s };
    let r = test.method();
    println!("{}", test.s); // OK since test is valid
    println!("{}", r.s); // Segfault (value ref'd by r dropped during index)
}
