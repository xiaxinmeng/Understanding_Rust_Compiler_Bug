
enum Foo {
    A(uint, uint), B(uint),
}

fn main() {
    match B(0) {
        A(_) => println!("A!"),
        B(_) => println!("B!"),
    }
}
