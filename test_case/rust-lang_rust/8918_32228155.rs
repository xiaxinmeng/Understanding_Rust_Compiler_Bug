
fn bar<'a>(x: &'a int) {
    println!("{:?}", x);
}

fn main() {
    bar(&42);
}
