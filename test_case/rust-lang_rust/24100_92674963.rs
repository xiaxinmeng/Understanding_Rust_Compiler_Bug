 rust
#[derive(Debug)]
struct S {
    v: String,
}

fn main() {
    let x = S { v: "hello".to_string() };
    println!("{:?}", x as S);
}
