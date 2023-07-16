 rust

trait Foo {
   fn foo(&mut self);
}

impl<'a> Foo for &'a mut Foo {
    fn foo(&mut self) { self.foo(); }
}

struct S {
    i :int
}


impl Foo for S {
    fn foo(&mut self) {
        self.i = self.i + 1;
    }
}

trait Bar {
    fn bar(&mut self);
}

impl<T: Foo> Bar for T {
    fn bar(&mut self) {
        self.foo();
    }
}

fn main() {
    let s = &mut S{i:0} as &mut Foo;
    s.foo(); // works fine
    s.bar(); // error: cannot borrow immutable local variable as mutable

    let mut s = &mut S{i:0} as &mut Foo;
    s.bar(); // works fine

}

