 rust
#[deriving(Show)]
struct Foo(int);

fn Bar(a: int) -> int { a }

type Bar = Foo;

fn main() {
    println!("{}", Bar(1));
}
