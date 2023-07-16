 rust
// This works:
let _: Rc<Send> = Rc::new(Bass(37)).clone();

// This also works:
let a: Rc<Bass> = Rc::new(Bass(37));
let _: Rc<Send> = a.clone();

// This triggers the bug:
let a: Rc<Bass> = Rc::new(Bass(37));
let b: Rc<Send> = a.clone();
drop(a);
drop(b);
