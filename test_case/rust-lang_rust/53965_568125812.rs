
error[E0271]: type mismatch resolving `<&&[&[u8; 1]; 1] as std::ops::Deref>::Target == [_]`
 --> src/main.rs:7:5
  |
3 | fn foo<Inner: Deref<Target=[u8]>, Outer: Deref<Target=[Inner]>>(buf: Outer) {}
  |    ---                                         -------------- required by this bound in `foo`
...
7 |     foo(&&[&data]);
  |     ^^^ expected slice, found `&[&[u8; 1]; 1]`
  |
  = note: expected slice `[_]`
              found type `&[&[u8; 1]; 1]`
