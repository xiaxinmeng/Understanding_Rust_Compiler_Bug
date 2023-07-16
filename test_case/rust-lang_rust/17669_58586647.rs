 rust
// Crate A

trait Foo<T> {
    fn foo(&self) -> &T;
}

fn use_foo<T, S: Foo<T>>(s: &S) -> &T {
    s.foo()
}

// Crate B, uses crate A

struct B;
impl Foo<B> for u8 { ... }

use_foo(0u8) // compiles, has type B

// Crate C, uses crate A

struct C;
impl Foo<C> for u8 { ... }

use_foo(0u8) // compiles, has type C

// Crate D, which uses A, B and C

use_foo(0u8) // does not compile, ambiguity 
