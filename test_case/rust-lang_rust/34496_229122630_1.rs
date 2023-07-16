
let c = RefCell::new(..);
{ let x: RefMut<_> = c.borrow_mut(); let x = &mut *x; /* use *x */ }
{ let y: RefMut<_> = c.borrow_mut(); let y = &mut *y; /* use *y */ }
