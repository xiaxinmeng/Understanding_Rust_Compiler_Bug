
enum Foo {
    A(uint, uint), B(uint),
}

fn main() {
    match B(0) {
        A(..) => println!("A!"),
        B(..) => println!("B!"),
    }
}
