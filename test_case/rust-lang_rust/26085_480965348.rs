rust
fn fn_once<'a, T: 'a, F: FnOnce() -> &'a mut T>(_: F) { }

fn main() {
    let mut slot = Box::new(1);
    fn_once(|| &mut slot);
}
