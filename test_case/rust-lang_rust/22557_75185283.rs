 rust
struct Nothing;

fn foo<'a, 'b, F: Fn(&'a Nothing) -> T, T: 'b>(_: F) { }
fn bar<'a, 'b, F: Fn(&'a Nothing) -> &'b Nothing>(_: F) { }

fn main() {
    bar(|x| x); // OK
    bar(|x: &Nothing| x); // OK
    foo(|x| x); // OK
    foo(|x: &Nothing| x); // Error
}
