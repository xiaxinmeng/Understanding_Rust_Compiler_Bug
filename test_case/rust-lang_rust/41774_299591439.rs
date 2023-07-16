rust
struct Data(i32);

impl Data {
    fn oh_no(&mut self, other: &mut Data) {
        // shouldn't ever succeed, but does
        assert!(std::ptr::eq(self, other));
    }
}

fn main() {
    let mut v = vec![Data(0)];
    v[0].oh_no(&mut v[0]);
}
