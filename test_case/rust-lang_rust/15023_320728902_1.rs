rust
fn main() {
    let x;
    {
        x = &String::from("foo");
        // `x` is dead here
        // Is the `String` dropped here?
    }
    // Or is it dropped here?
}
