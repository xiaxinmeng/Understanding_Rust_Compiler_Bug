rust
fn main() {
    let f = Foo { f: 0, g: 1 };
    match f {
        const { ZERO(22) } => println!("hi"), // <-
        _ => println!("1"),
    }
}
