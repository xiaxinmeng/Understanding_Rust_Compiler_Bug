
error[E0391]: cycle detected when computing layout of `Foo`
  |
note: ...which requires const-evaluating `Foo::{{initializer}}`...
 --> ../../par-merge/src/test/compile-fail/issue-44415.rs:19:26
  |
19|     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...which again requires computing layout of `Foo`, completing the cycle
