rust
let r = RefCell::new(42);
let r1 = &r;
let r2 = &r;
let _mutref = r1.borrow_mut();
let illegal_copy = *r2; // copying while there is an outstanding mutable reference
