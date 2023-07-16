rust
fn foo(mut f: Box<FnMut()>) {
    f();
}

fn main() {
    let x = vec![];
    x.push(true);
    let y = true;
    foo(Box::new(move || y = false) as Box<_>);
}
