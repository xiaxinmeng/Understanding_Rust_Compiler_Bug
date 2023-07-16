 rust
trait Callable {
    fn call(self);
}

impl Callable for fn() {
    fn call(self) {
        self();
    }
}

fn call<F: Callable>(f: F) {
    f.call();
}

fn foo() {
    println!("foo");
}

fn main() {
    call(foo); // error: the trait `Callable` is not implemented for the type `fn() {foo}`
}
