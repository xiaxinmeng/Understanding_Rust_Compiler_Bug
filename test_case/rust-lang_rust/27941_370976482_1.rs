rust
trait MyTrait {
    fn foo(self: &Rc<Self>, arg: String);
}

struct Thing;

impl MyTrait for Thing {
    fn foo(self: &Rc<Self>, arg: String) {
        unimplemented!()
    }
}
