rust
fn boo(b: Option<B<&'static str>>) {
    // let b1 = b; // uncommenting this brings the hang back.
}
