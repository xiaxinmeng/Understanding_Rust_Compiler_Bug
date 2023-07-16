rust
let mut foo = Rc::new(RefCell::new(None));
let bar = Rc::new(RefCell::new(Some(foo.clone())));
*foo.borrow_mut() = Some(bar);
