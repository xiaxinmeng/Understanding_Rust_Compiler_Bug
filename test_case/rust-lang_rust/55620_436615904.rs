rust
#[derive(Debug)]
struct Foo;

fn main() {
    let v = vec![Some(Foo)];

    if let Some(ref pat) = v[0] {
        println!("{:?}", pat);
    }
}
