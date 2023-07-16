
error[E0391]: unsupported cyclic reference between types/traits detected
    |
note: the cycle begins when computing layout of `Foo`...
note: ...which then requires const-evaluating `Foo::{{initializer}}`...
   --> b.rs:5:17
    |
5   |     bytes: [u8; std::mem::size_of::<Foo>()],
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: ...which then again requires computing layout of `Foo`, completing the cycle.

error: aborting due to previous error
