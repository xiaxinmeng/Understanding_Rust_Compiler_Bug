 rust
let targets: Rc<RefCell<Vec<Box<Observer<Time=T, Data=D>>>>> = Rc::new(RefCell::new(Vec::new()));
self.add_observer(OutputPort { shared: targets.clone() });
