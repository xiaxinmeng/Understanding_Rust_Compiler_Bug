Rust
fn foo<F: Fn(&u32)>(f: F) { let x = 42; f(&x); }
fn bar<'a>(_x: &'a u32) {
    foo(|_y: &'a u32| {}); // this should be an error: `_y` isn't an `&'a u32`
}

fn main() {
    bar(&0);
}
