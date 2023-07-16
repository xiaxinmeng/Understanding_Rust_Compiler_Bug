rust
let a: Rc<Vec<_>> = ...;
Rc::make_mut(&mut a).pop(); // make_mut is stable
