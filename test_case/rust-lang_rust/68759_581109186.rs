rust
pub trait A {}

pub struct B {
    a: Box<dyn A>,
}

impl B {
    pub fn new(a: Box<dyn A>, x: &(), y: &()) -> B {
        B { a }
    }
}
