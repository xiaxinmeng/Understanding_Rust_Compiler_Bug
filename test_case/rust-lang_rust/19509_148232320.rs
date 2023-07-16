
impl Bar {
    fn foo(self: Box<Rc<Self>>) // admittedly, not legal yet, but the code is trying to be prepared for it
}
