rust
use std::cell::RefMut;
fn derefs_ref_mut(x: RefMut<'_, u32>) {
    *x = 1;
}
