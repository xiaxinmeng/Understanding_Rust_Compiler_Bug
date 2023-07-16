
use std::ops::DerefMut;
fn main() {
    let mut x = Box::new((1, 2));
    { let (ref mut a, ref mut b) = *&mut *x; }
    { let (ref mut a, ref mut b) = *x.deref_mut(); }
}
