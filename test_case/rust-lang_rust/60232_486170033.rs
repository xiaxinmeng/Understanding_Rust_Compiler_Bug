rust
This error indicates that you are trying to borrow a variable as mutable when it
has already been borrowed as immutable.

Example of erroneous code:


fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    let ref y = a; // a is borrowed as immutable.
    bar(a); // error: cannot borrow `*a` as mutable because `a` is also borrowed
            //        as immutable
}


To fix this error, ensure that you don't have any other references to the
variable before trying to access it mutably:


fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    bar(a);
    let ref y = a; // ok!
}

