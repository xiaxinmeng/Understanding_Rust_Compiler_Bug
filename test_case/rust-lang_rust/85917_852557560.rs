rust
trait Foo {
    fn make_it() -> Self where Self: Sized;
}

impl Foo for String {
    fn make_it() -> Self {
        String::new()
    }
}

fn main() {
    let val: String = {
        let tmp = Foo::make_it();
        tmp.len();
        tmp
    };
}
