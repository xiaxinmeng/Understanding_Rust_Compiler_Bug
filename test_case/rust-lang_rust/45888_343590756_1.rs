
error[E0389]: cannot borrow data mutably in a `&` reference
  --> foo-expanded.rs:71:33
   |
68 |             Foo(ref __self_1_0) =>
   |                 -------------- consider changing this to `ref mut __self_1_0`
...
71 |                 (*__self_0_0) < (*__self_1_0) ||
   |                                 ^^^^^^^^^^^^^ assignment into an immutable reference

error[E0389]: cannot borrow data mutably in a `&` reference
  --> foo-expanded.rs:72:39
   |
70 |                 Foo(ref __self_0_0) =>
   |                     -------------- consider changing this to `ref mut __self_0_0`
71 |                 (*__self_0_0) < (*__self_1_0) ||
72 |                     !((*__self_1_0) < (*__self_0_0)) && false,
   |                                       ^^^^^^^^^^^^^ assignment into an immutable reference
