 rust
struct A;
impl A {
    fn foo(a: &A) {}
}

let a = A;
a.foo(); // error
A::foo(&a); // ok
