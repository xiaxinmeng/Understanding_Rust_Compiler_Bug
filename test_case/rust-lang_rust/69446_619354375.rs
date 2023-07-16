rust
let x = Rc::new(RefCell::new(0));
(async || *(x.borrow_mut()) += 1)().await;
