
fn foo() -> Box<i32> {
    Box::new(42)
}

fn bar<F: FnOnce()->Box<dyn std::fmt::Debug>>(f: F) {
    println!("{:?}", f())
}

fn main() {
    bar(||foo()); // this works
    // bar(foo); // this doesn't work
}
