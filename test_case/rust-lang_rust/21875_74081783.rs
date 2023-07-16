 rust
fn main() {
    let foo: Box<&i32>;
    let bar = 1337;
    foo = Box::new(&bar);
    { foo; } // kill foo
    foo; // error expected, foo was moved
    bar; // no error, bar is still alive
}
