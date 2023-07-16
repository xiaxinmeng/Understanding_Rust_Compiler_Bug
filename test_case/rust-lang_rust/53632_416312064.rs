rust
// no copy, no clone!
#[derive(Debug)]
struct Foo(i32);

fn my_example(foos: Vec<Foo>) {
    let mut foos_iter = foos.into_iter();
    for x in foos_iter.by_ref().take(6) {
        println!("First: {:?}", x);
    }
    for x in foos_iter {
        println!("Rest: {:?}", x);
    }
}
fn main() {
    my_example((0..12).map(Foo).collect());
}
