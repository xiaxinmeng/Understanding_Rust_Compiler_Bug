rust
// fn a(a: &str) -> &str {
fn a(a: &str) -> &'static str {
    ""
}

fn main() {
    let mut foo = BoxHolder { data: Box::new(a) };
    foo.box_and_set_data(a);
}
