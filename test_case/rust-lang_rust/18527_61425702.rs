
trait T {
    fn foo();
}

fn bar<X: T>(x: Box<T>) {
    X::foo();
}

fn main() {
    let y: Box<T> = ...;
    bar(y);
}
