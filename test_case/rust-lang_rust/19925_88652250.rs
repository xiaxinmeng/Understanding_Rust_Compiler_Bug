 rust
fn foo<A>(_: A, _: A) { }

fn bar<B>() { }

fn main() {
    let x = bar::<u32>;
    let y = bar::<i32>;
    foo(x, y);
}
