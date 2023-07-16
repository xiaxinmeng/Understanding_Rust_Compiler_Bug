rust
fn main() {
    let x = || ();
    Fn::call(&x, ());
}
