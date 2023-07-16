 rust
trait MyTrait {
    fn do_something(&self);
}

impl<'a> MyTrait for MyTrait + 'a {
    fn do_something(&self) {
        println!("MyTrait");
    }
}

fn main() {
}
