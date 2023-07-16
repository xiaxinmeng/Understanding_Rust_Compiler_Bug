 rust
struct MyStruct;

fn main() {
    let v1: Vec<MyStruct> = Vec::new();
    let v2: &[MyStruct] = (&v1).clone();
}
