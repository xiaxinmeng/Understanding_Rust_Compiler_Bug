 rust
trait Foo {
    fn do_foo(&self);
}

trait Bar {
    fn do_bar(&self);
}

impl<T> Bar for T where T: Foo {
    fn do_bar(&self) {
        self.do_foo()
    }
}

struct Obj {
    obj: Box<Bar + 'static>
}

impl Obj {    
    fn process(&self) {
        (*self.obj).do_bar();
    }
}
