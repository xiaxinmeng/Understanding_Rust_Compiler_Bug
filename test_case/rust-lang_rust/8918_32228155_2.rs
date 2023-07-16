
struct Foo<'a> {
    x: &'a int
}

fn bar<'a>(x: &'a int) {
    println!("{:?}", Foo {x:x});
}

fn main() {
    bar(&42);
}
