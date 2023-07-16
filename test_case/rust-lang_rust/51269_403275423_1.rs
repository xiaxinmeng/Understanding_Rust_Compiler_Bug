rust
fn main() {
    let f: &'static (); {
        loop {break};
        f = &() // () is a constant expression, so can promote to static
    }
}
