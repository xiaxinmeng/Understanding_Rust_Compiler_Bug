 rust
pub trait Factory {
    type W: Widget;

    fn new_widget(&self) -> Self::W;
}

pub trait Widget {
    fn turn(&mut self);
}

pub struct FailFactory {
    factory: Box<Factory>
}

impl FailFactory {
    /*
    pub fn fictious_type_fail(&mut self) {
    self.factory.new_widget();
    }
    */
    pub fn ty_projection_fail(&mut self) {
        let mut widget = self.factory.new_widget();
        widget.turn();
    }
}

fn main() {}
