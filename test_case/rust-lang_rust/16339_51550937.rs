 rust
fn small(x: &mut ()) -> &mut () {
    (|| &mut *x)()
}


fn main() { }
