rust
pub struct BoxHolder<BoxedFn> {
    data: BoxedFn,
}

impl<Ret> BoxHolder<Box<fn(&str) -> Ret>> {
    pub fn box_and_set_data(&mut self, ptr: fn(&str) -> Ret) {
        let pre_boxed: Box<fn(&str) -> Ret> = Box::new(ptr); // box contents have concrete size, type annotations are for clarity
        self.data = pre_boxed; // not the case here
    }
}

fn a(a: &str) -> &str {
    ""
}

fn main() {
    let mut foo: BoxHolder<Box<fn(&str) -> &str>> = BoxHolder { data: Box::new(a) };
    foo.box_and_set_data(a);
}
