
fn foo<T: 'static>(_: T) {}

fn bar<T: 'static>(x: &'static T) {
    foo(x);
}
fn main() {}
