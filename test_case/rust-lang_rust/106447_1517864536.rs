rust
trait T {}
impl T for u8 {}
impl T for Foo {}

#[repr(transparent)]
struct Foo(u8);

let f = Foo(1);
let p1: *const dyn T = &f;
let p2: *const dyn T = &f.0;

assert_ne!(p1, p2); // FAILS IN RELEASE MODE, passes in debug mode
