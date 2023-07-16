 rust
pub struct OutputPort<T: Timestamp, D: Data> {
    pub shared:    Rc<RefCell<Vec<Box<Observer<Time=T, Data=D>>>>>,
}
