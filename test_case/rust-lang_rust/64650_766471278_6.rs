rust
impl BoxHolder<Box<for<'r> fn(&'r str) -> &'static str>> {
    pub fn box_and_set_data(&mut self, ptr: for<'r> fn(&'r str) -> &'static str) {
        self.data = Box::new(ptr);
    }
}

fn a(a: &str) -> &'static str {
    ""
}

fn main() {
    let mut foo = BoxHolder { data: Box::new(a) };
    foo.box_and_set_data(a);
}
