 rust
struct Foo {
     a:int
}

trait FooUtil {
    fn add(self) -> int;
}

impl FooUtil for Foo {
    fn add(self) -> int {
        2
    }
}

fn main() {
    let a  = Foo { a:1 };
    a.add();
}
