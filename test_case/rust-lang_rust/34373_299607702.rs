
error[E0391]: unsupported cyclic reference between types/traits detected
 --> test.rs:6:19
  |
6 | type DefaultFoo = Foo;
  |                   ^^^ cyclic reference
  |
note: the cycle begins when processing `Foo::T`...
 --> test.rs:5:16
  |
5 | pub struct Foo<T = Box<Trait<DefaultFoo>>>;
  |                ^
note: ...which then requires processing `DefaultFoo`...
 --> test.rs:5:30
  |
5 | pub struct Foo<T = Box<Trait<DefaultFoo>>>;
  |                              ^^^^^^^^^^
  = note: ...which then again requires processing `Foo::T`, completing the cycle.

error: aborting due to previous error
