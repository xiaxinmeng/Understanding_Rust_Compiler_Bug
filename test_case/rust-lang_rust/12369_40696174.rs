 Rust
fn main() {
    enum List {
        Cons(int, ~List),
        Nil
    }

    let x = ~Cons(1, ~Nil);
    match *x {
        Cons(a, ~ref next) => (),
        Cons(b, ~Nil) => (),
        _ => ()
    }
}
