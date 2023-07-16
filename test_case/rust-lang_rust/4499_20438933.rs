
use std::cast;
use std::io;
enum Void { }
struct Foo;
fn main() {
    let x: Void = unsafe { cast::transmute(Foo) };
    io::println(fmt!("%?", x));
}
