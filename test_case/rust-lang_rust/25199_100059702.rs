 rust
use std::cell::RefCell;

struct VecHolder {
    v: Vec<u32>,
}

impl Drop for VecHolder {
    fn drop(&mut self) {
        println!("Dropping Vec");
    }
}

trait Foo {}

impl<'a> Foo for Test<'a> {}

struct Container<'a> {
    v: VecHolder,
    d: RefCell<Vec<Box<Foo + 'a>>>,
}

impl<'a> Container<'a> {
    fn new() -> Container<'a> {
        Container{d: RefCell::new(Vec::new()), v: VecHolder{v: vec![42; 100]}}
    }

    fn store(&'a self, val: Box<Foo + 'a>) {
        self.d.borrow_mut().push(val);
    }
}

struct Test<'a> {
    test: &'a Container<'a>,
}

impl<'a> Drop for Test<'a> {
    fn drop(&mut self) {
        println!("Val from Vec: {}", self.test.v.v[30]);
    }
}

fn main() {
    let (container, test);
    container = Container::new();
    test = Test{test: &container};
    container.store(Box::new(test) as Box<Foo>);
}
