
error[E0599]: no method named `get` found for type `&mut std::rc::Rc<std::cell::RefCell<std::collections::HashMap<&str, i32>>>` in the current scope
  --> src/main.rs:34:30
   |
34 |     let v1 = rc.borrow_mut().get("myKey1").unwrap().clone();
   |                              ^^^
