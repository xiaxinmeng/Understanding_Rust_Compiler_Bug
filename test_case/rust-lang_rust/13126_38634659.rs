 rust
let foo = Rc<Foo>;
foo.downgrade(); // calls Foo's downgrade
let weak = Rc::downgrade(foo); // the only way to call Rc's downgrade
