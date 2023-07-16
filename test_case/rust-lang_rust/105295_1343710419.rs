rust
pub struct Listeners<'a> {
    listeners: RefCell<Vec<Box<dyn FnMut(()) + 'a>>>
}

pub trait ListenersInterface {
    fn listeners<'b>(&'b self) -> &'b Listeners<'b>;
}

impl<'a> ListenersInterface for Listeners<'a> {
    fn listeners<'b>(&'b self) -> &'a Listeners<'b> {
        self
    }
}
