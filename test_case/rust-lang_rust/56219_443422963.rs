Rust
fn let_expansion() -> Box<Error> {
    let x = Box::new(!);
    x /* should this work? */
}
