 rust
trait Foo: Send {
    pub fn sendit(self) {
        do spawn {
            printfln!("%?", self);
        }
    }
}

fn foo<T: Foo>(x: T) {
    x.sendit();
}

impl Foo for int;

fn main() {
    let x = 5i;
    foo(x);
}
