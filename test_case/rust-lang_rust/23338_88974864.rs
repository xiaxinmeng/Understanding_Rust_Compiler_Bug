 rust
fn foo(x: RefCell<String>) -> String {
  let t = x.borrow().clone();
  t // this works!
}
