
struct Foo;

fn show<T: std::fmt::Debug = Foo>(t: T) {
    println!("{:?}", t)
}

fn main() {
    let f = Foo;
    show(f);
}
