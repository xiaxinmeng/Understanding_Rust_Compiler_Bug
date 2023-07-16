rust
use std::ops::Deref;

struct A;
struct B;

impl Deref for A {
    type Target = B;
    fn deref(&self) -> &B {
        &B
    }
}

// Replace the above with these and it will work:
//type A = String;
//type B = str;

struct E;

fn foo(a: Result<A, E>) -> Result<(), E> {
    // &*a? or &&a? or a?.deref() also work
    bar(&a?);
    Ok(())
}

fn bar(_: &B) {}
