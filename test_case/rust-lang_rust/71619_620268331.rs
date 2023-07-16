rust
fn foo(_: Option<fn()>) {}
fn bar() {}

pub fn main() {
    foo(Some(bar)); // Works

    let f: Option<fn()> = Some(bar);
    foo(f); // also works as expected

    let f = Some(bar);
    foo(f); // error[E0308]: mismatched types expected fn pointer, found fn item
}
