
fn foo() { println!("hi"); }

fn bar(x: ||: 'static) {
    (x)();
    let y = x.clone();
    (y)();
}

fn main() {
    bar(foo);
}
