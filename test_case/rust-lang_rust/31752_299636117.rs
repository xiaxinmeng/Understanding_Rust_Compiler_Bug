rust
fn foo(mut f: Box<FnMut()>) {
    f();
}

fn main() {
    let y = true;
    foo(Box::new(move || y = false) as Box<_>);
}
