rust
#[derive(Debug)]
struct S {
    f: usize,
}

fn main() {
    let s = S { f: 0 };
    println!("{:?}", s);
}
