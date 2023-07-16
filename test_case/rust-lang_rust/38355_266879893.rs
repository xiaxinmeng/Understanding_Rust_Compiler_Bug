rust
let x = RefCell::new(vec![1,2,3]);

match x.borrow().len() {
 ...
}
