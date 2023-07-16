
error[E0309]: the parameter type `K` may not live long enough
 --> src/lib.rs:3:39
  |
3 | fn foo<'a, K, V, I: 'a + IntoIterator<Item = (&'a K, &'a V)>>(i: I) {}
  |            -                          ^^^^^^^^^^^^^^^^^^^^^
  |            |
  |            help: consider adding an explicit lifetime bound...: `K: 'a`
  |
note: ...so that the reference type `&'a K` does not outlive the data it points at
 --> src/lib.rs:3:39
  |
3 | fn foo<'a, K, V, I: 'a + IntoIterator<Item = (&'a K, &'a V)>>(i: I) {}
  |                                       ^^^^^^^^^^^^^^^^^^^^^

error[E0309]: the parameter type `V` may not live long enough
 --> src/lib.rs:3:39
  |
3 | fn foo<'a, K, V, I: 'a + IntoIterator<Item = (&'a K, &'a V)>>(i: I) {}
  |               -                       ^^^^^^^^^^^^^^^^^^^^^
  |               |
  |               help: consider adding an explicit lifetime bound...: `V: 'a`
  |
note: ...so that the reference type `&'a V` does not outlive the data it points at
 --> src/lib.rs:3:39
  |
3 | fn foo<'a, K, V, I: 'a + IntoIterator<Item = (&'a K, &'a V)>>(i: I) {}
  |                                       ^^^^^^^^^^^^^^^^^^^^^
