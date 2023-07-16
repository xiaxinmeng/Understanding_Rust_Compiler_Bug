
error[E0391]: unsupported cyclic reference between types/traits detected
   --> /scratch/dsprenkels/rust/src/libcore/mem.rs:315:14
    |
315 |     unsafe { intrinsics::size_of::<T>() }
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ cyclic reference
    |
note: the cycle begins when computing layout of `Foo`...
   --> /scratch/dsprenkels/rust/src/libcore/mem.rs:315:14
    |
315 |     unsafe { intrinsics::size_of::<T>() }
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which then requires const-evaluating `Foo::{{initializer}}`...
   --> issue-44415.rs:14:17
    |
14  |     bytes: [u8; std::mem::size_of::<Foo>()],
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: ...which then again requires computing layout of `Foo`, completing the cycle.

error: aborting due to previous error
