
struct Foo;

trait Bar {
    fn bar(&self) -> int;
}
impl Bar for Foo {
    fn bar(&self) -> int {1}
    fn bar(&self) -> int {2}
}

fn main() {
    println!("{}",Foo.bar());
}
