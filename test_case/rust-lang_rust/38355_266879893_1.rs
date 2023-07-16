rust
let x = RefCell::new(vec![1,2,3]);

match x.borrow().index(1) { // returns an &u8, lifetime tied to borrow
 ...
}
