rust
impl MyTrait for Thing {
    fn foo(self: &Rc<MyTrait>, inner_self: &Self, arg: String) {
        unimplemented!()
    }
}
