rust
fn main() {
    let mut foo = Some("foo".to_string());
    let bar = &mut foo;
    match bar {
        Some(ref mut baz) => {
            bar.take();
            drop(baz);
        },
        None => unreachable!(),
    }
}
