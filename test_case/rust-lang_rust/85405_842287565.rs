rust
fn main() {
    Hello::new(|_: &mut ()| ()).hello();
    //           ^^^^^^^^^
}
