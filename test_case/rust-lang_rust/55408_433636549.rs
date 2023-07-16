
pub struct Rc<T>(T);

pub struct Foo;


impl Rc<Foo> {
    pub fn bar() {}
}

