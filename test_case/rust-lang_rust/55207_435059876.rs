rust
let x = RefCell::new(2);
let mut y = x.borrow_mut();
let z: &mut u32 = &mut *y;
let copy = *x.as_unsafe_cell(); // reads from `z`, but not through `z`
