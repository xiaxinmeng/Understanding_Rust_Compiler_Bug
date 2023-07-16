rust
mod detail {
    #[derive(Debug)]
    pub enum Foo {
        Var1,
        Var2
    }
}
use detail::Foo;

mod bar {
    use super::Foo;

    pub fn get_foo() -> Foo {
        Foo::Var1
    }
}

fn main() {
    println!("{:?}", bar::get_foo());
    println!("Hello, world!");
}
