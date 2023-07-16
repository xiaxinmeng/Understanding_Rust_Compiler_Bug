
use std::mem;

trait Foo{
    fn foo(&mut self);
    fn bar(&self)->isize;
}
impl Foo for isize{
    fn foo(&mut self) {
        *self=*self+1
    }
    fn bar(&self)->isize{
        *self
    }
}

fn main() {
unsafe{
    let (a1,a2) = mem::transmute::<Box<Foo>,(usize,usize)>(Box::new(1isize) as Box<Foo>);
    let (b1,b2) = mem::transmute::<&mut Foo,(usize,usize)>(&mut 2isize as &mut Foo);
    let mut a = mem::transmute::<(usize,usize),Box<Foo>>((b1,a2));
    let b = mem::transmute::<(usize,usize),&mut Foo>((a1,b2));
    a.foo();b.foo();
    println!("{:?}", (a.bar(),b.bar()))
}
}

