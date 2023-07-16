rust
#[derive(Debug)]
#[repr(transparent)]
struct Foo(u8);

let f = Foo(1);
let p1: *const dyn Debug = &f;
let p2: *const dyn Debug = &f.0;

assert_ne!(p1, p2); // always passes today
