rust
#[derive(Debug)]
struct Foo {
    pub a: i32,
}

let a = Foo {
    a: 0
};
let b = Foo{..a};

println!("{:?} {:?}", a, b);
}
