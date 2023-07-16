rust
fn main() {
    let foo = (Box::new(22), Box::new(44));
    match foo { (_, y) => () }
    drop(foo.0); // ok, foo.0 was not moved
}
