 rust
struct Type;

trait A {
    fn foo(&self) -> bool { false }
}

trait B : Sized {
    fn foo(self) -> bool { true }
}

impl A for Type { }
impl B for Type { }

fn main() {
    println!("{}", Type.foo());   // This will call B::foo -- it will prefer `self`.
}
