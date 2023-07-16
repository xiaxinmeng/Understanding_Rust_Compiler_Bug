 rust
fn foo<T: Fn()>(_: T) {}

fn main() {
    let a = Box::new(1);
    let x = || {
        a;
    };
    foo(x);
}
