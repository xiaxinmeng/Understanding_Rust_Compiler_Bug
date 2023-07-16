rust
#[derive(Debug)]
struct Foo;

fn main() {
    let x = Foo;
    let f = move || println!("{:?}", x);
    assert_eq!(std::mem::size_of_val(&f), 0);
}
