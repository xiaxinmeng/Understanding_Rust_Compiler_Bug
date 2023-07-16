
trait Foo {}
impl Foo for int {}

fn main() {
    let box x = box 1i as Box<Foo>;
    let &x = &1i as &Foo;
}
