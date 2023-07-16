rust
struct StructA {
    pub a: u32,
}

fn main() {
    StructA { a: 0 }.a = 10;
    println!("{}", StructA { a: 0 }.a);
}
