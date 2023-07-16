rust
#[derive(Debug)]
struct A {
    x: i32,
    y: i32,
}

fn main() {
    println!("{:#010?}", A { x: 1, y: 2 });
}
