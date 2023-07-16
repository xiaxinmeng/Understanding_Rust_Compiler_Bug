rust
fn test() {
   fn nested() {}
}

fn main() {
    nested(); // dg-error etc...
}
