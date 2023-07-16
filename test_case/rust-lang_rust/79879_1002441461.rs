
error[E0623]: lifetime mismatch
  --> src/lib.rs:12:34
   |
11 | fn errors(a: &mut A) {
   |              ------
   |              |
   |              these two types are declared with different lifetimes...
12 |     let mut b = child_with_value(a);
   |                                  ^ ...but data from `a` flows into `a` here
   = note: each elided lifetime in input position becomes a distinct lifetime
help: explicitly declare a lifetime and assign it to both
   |
11 | fn errors<'a>(a: &'a mut A<'a>) {
   |          ++++     ++      ++++
