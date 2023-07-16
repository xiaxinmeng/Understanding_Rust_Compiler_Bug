 rust
#[deriving(Show)]
mod Foo {
    pub static X: int = 42;
}
type Foo = u32;

fn main() {
    println!("{}", Foo::X);
}
