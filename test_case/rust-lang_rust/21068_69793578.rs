 rust
let targets = Rc::new(RefCell::new(Vec::new()));
self.add_observer(OutputPort { shared: targets.clone() });
