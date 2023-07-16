


rustc 1.15.1 (021bd294c 2017-02-08)
error[E0308]: mismatched types
  --> <anon>:19:9
   |
19 |         0usize..10
   |         ----------
   |         |
   |         expected associated type, found struct `std::ops::Range`
   |         in this macro invocation
   |
   = note: expected type `<&'a S as Iter>::Type`
   = note:    found type `std::ops::Range<usize>`

error: aborting due to previous error

