
use std::mem::transmute;
trait Foo {
    fn foo(&self) {}
}
impl Foo for u32 {}
impl Foo for i32 {}

fn main() {
unsafe {
    println!("{:?}", (transmute::<Box<Foo>, (usize,usize)>(Box::new(1) as Box<Foo>),
    transmute::<&Foo, (usize,usize)>(&1 as &Foo),
    transmute::<Box<Foo>, (usize,usize)>(Box::new(2) as Box<Foo>),
    transmute::<&Foo, (usize,usize)>(&2 as &Foo)));
}
}
