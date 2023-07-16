rust
let p = &RefCell::new(22);
let q = p.borrow_mut();
drop(p);
drop(q);
