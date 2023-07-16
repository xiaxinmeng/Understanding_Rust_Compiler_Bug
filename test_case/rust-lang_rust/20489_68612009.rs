 rust
#[deriving(PartialEq)]
struct Foo<T> {
    value: T
}

const F: Foo<int> = Foo { value: 2 };

fn main() {
    let x = Foo { value: 2 };
    match x {
        _ if x == F => { println!("Hi"); }
        _ => { println!("Ho"); }
    }
}
