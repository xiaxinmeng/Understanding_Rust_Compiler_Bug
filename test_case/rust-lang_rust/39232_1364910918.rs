
error[E0308]: mismatched types
 --> src/test/ui/suggestions/shadowed-lplace-method.rs:9:24
  |
9 |     *rc.borrow_mut() = false; //~ ERROR E0308
  |     ----------------   ^^^^^ expected struct `Rc`, found `bool`
  |     |
  |     expected due to the type of this binding
  |
  = note: expected struct `Rc<RefCell<bool>>`
               found type `bool`
note: there are multiple methods with the same name, `borrow_mut` refers to `std::borrow::BorrowMut::borrow_mut` in the method call
 --> src/test/ui/suggestions/shadowed-lplace-method.rs:9:9
  |
9 |     *rc.borrow_mut() = false; //~ ERROR E0308
  |         ^^^^^^^^^^ refers to `std::borrow::BorrowMut::borrow_mut`
help: you might have meant to invoke a different method, you can use the fully-qualified path
  |
9 |     *std::cell::RefCell::<_>::borrow_mut(&rc) = false; //~ ERROR E0308
  |      +++++++++++++++++++++++++++++++++++++  ~
