
use std::default::Default;

pub trait Message {
    fn new() -> Self { panic!(); }
}

fn new_instance<M : 'static + Message>() -> Box<Message + 'static> {
    let m: M = Default::default();
    box m as Box<Message>
}

fn main() {}
