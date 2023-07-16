rust
#![feature(trait_alias)]

trait Foo {
    fn foo(&self) -> u32;
}

trait SendAlias = Send;

impl Foo for dyn Send {
    fn foo(&self) -> u32 {
        1
    }
}

impl Foo for dyn SendAlias {
    fn foo(&self) -> u32 {
        2
    }
}

fn main() {
    let x1 = Box::new(()) as Box<Send>;
    let x2 = Box::new(()) as Box<SendAlias>;
    println!("{}", x1.foo());
    println!("{}", x2.foo());
}
