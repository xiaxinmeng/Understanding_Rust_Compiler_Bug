rust
let a = container.item as *const _ as *const ();
let b = &item as *const _ as *const ();
assert!(ptr::eq(a, b));
