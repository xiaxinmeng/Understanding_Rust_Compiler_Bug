
fn foo() {
    task::yield();
    task::spawn(foo);
}

fn main() {
    let b = task::builder();
    task::unsupervise(b);
    task::run(b, foo);
    for iter::repeat(1000) { task::yield(); }
    fail;
}
