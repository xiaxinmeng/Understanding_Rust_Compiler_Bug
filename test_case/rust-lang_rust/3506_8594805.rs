
enum Foo = uint;

fn main() {
    let x = Foo(0);
    let y = Foo(1);
    io::println(fmt!("%?, %?", x, y));
}
