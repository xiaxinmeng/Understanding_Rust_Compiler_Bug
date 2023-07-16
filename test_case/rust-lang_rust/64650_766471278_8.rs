rust
pub struct BoxHolder<BoxedFn> {
    data: BoxedFn,
}

impl BoxHolder<Box<fn(&str) -> &str>> {
    pub fn box_and_set_data(&mut self, ptr: fn(&str) -> &str) {
        self.data = Box::new(ptr);
    }
}

fn a(a: &str) -> &str {
    ""
}

fn main() {
    let mut foo: BoxHolder<Box<fn(&str) -> &str>> = BoxHolder { data: Box::new(a) };
    foo.box_and_set_data(a);
}
