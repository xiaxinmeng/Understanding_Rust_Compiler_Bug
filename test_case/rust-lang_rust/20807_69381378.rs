 rust
#![feature(unsafe_destructor)]

#[derive(Show)] struct Foo { foo: u8 }

#[unsafe_destructor]
impl<'a, 'b> Drop for Foo {
    fn drop(&mut self) {
        println!("Foo: {:?}", self.foo);
    }
}

/// Invariant: *x = 2 after this is called.
fn foo<'a>(x: &'a mut u8) {
    if *x == 0 {
        panic!("oops");
    }
    *x = 2;
}

fn main() {
    let mut x = Foo {foo: 0 };
    {
        foo(&mut x.foo);
    }
}
