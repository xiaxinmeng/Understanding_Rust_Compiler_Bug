rust
#![feature(never_type)]

struct A;

struct B;

trait Foo {
    type Output;

    fn foo(&self) -> Self::Output;
}

impl Foo for ! {
    type Output = A;

    fn foo(&self) -> A {
        A
    }
}

impl Foo for i32 {
    type Output = B;

    fn foo(&self) -> B {
        B
    }
}

fn main() {
    //let x = todo!().foo(); // Works
     let x = { todo!() }.foo(); // Doesn't work
}
