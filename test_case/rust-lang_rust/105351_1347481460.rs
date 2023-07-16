rust
fn main() {
    let a = || ();
    let b = || Err(a);

    let y = if true {
        b()
    } else {
        Ok(b())
    };
}
