rust
#[derive(Hash)]
struct Foo {
    a: Vec<u8>,
    b: Vec<u8>,
}

let thing1 = Foo {
    a: vec![1, 2],
    b: vec![3, 4],
};

let thing2 = Foo {
    a: vec![1],
    b: vec![2, 3, 4],
};
