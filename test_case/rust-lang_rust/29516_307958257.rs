rust
#![feature(optin_builtin_traits)]

trait NotSame {}
impl NotSame for .. {}
impl<A> !NotSame for (A, A) {}

trait OneOfEach {}

impl<A> OneOfEach for (A,) { }

impl<A, B> OneOfEach for (A, B)
    where (B,): OneOfEach,
          (A, B): NotSame { }

// ...

fn main() {}
