rust
struct Foo {
    x: i32,
}

fn main() {
    let mut foo = Foo { x: 42 };

    let x = &mut foo.x;
    *x = 13;
    let y = foo;
    println!("{:?}", (&y).x);  //only added this line
    println!("{:?}", y.x); //13
}
