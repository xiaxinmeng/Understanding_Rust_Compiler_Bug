rust
fn foo() {}

fn main() {
    let _f: fn() = foo; // Works
    let _fref: &fn() = &foo; //~ERROR expected fn pointer, found fn item
}
