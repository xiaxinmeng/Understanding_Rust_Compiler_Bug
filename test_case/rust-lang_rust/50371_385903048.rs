rust
use std::ptr::write_volatile;

enum Some {
    A,
    B,
}

trait FooTrait<T> {
    fn get(&mut self) -> Option<T>;
}

struct FooStruct<T> {
    some: Option<T>
}

impl<T> FooTrait<T> for FooStruct<T> {
    fn get(&mut self) -> Option<T> {
        self.some.take()
    }
}

fn main() {
    let foo: Box<FooTrait<Some>> = Box::new(FooStruct { some: Some(Some::A) });
    let mut a = Some(foo);
    unsafe {
        write_volatile(&mut a, None);
    }
}

