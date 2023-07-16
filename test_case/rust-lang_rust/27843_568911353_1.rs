text
error[E0308]: mismatched types
 --> src/main.rs:9:52
  |
9 |     let c1: Weak<RefCell<dyn Baz>> = Rc::downgrade(&a);             // Doesn't work
  |                                                    ^^ expected trait Baz, found i32
  |
  = note: expected type `&std::rc::Rc<std::cell::RefCell<dyn Baz>>`
             found type `&std::rc::Rc<std::cell::RefCell<i32>>`
