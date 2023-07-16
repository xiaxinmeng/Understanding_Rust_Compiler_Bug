rust
fn main() {
    // let mut foo: BoxHolder<Box<fn(&str) -> &str>> = BoxHolder { data: Box::new(a) };
    let mut foo = BoxHolder { data: Box::new(a) };
    foo.box_and_set_data(a);
}
