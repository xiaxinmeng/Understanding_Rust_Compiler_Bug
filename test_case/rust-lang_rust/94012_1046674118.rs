rust
struct DropsA<'a>(&'a str);
impl DropsA<'_> {
    fn as_unit(&self) {}
}
impl Drop for DropsA<'_> {
    fn drop(&mut self) {}
}

fn main() {
    let mut slot = None;
    drop(slot.replace(DropsA(&String::from("Hey"))))
}
