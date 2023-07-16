 rust
struct Grr<'a> {
    r: &'a Box<i32>
}

impl<'a> Drop for Grr<'a> {
    fn drop(&mut self) {
        println!("dropping {}", *self.r);
    }
}

#[test]
fn good_borrow01() {
    let mut r;
    let i = box 10i32;
    r = Grr { r: &i };
    assert_eq!(**r.r, 10);
}
