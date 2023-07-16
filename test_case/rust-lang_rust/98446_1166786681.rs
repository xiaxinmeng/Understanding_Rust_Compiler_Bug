rust
#[derive(Hash)]
struct A;

#[derive(Hash)]
#[repr(packed)]
struct Foo(A);

#[derive(Hash)]
#[repr(packed)]
pub struct Bar<T: Copy>(T);

fn main() {}
