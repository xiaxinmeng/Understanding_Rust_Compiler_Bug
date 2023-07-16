 rust
#[derive(Copy)]
struct Foo;

impl Clone for Foo { fn clone(&self) -> Self { panic!() } }

// Will panic in new code, used to be “fine.”
[Foo].clone(); 
