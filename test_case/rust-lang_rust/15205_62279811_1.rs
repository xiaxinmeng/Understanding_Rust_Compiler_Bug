 rust
type Foo = u32;

#[deriving(Show)]
mod Foo {
    pub static X: int = 42;
}

fn main() {
    println!("{}", Foo::X);
}
