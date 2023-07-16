
error[E0597]: `b` does not live long enough
  --> src/lib.rs:21:12
   |
19 | fn errors<'a>(a: &'a mut A<'a>) {
   |           -- lifetime `'a` defined here
20 |     let mut b = a.child_with_value("foo".into());
21 |     errors(&mut b);
   |     -------^^^^^^-
   |     |      |
   |     |      borrowed value does not live long enough
   |     argument requires that `b` is borrowed for `'a`
22 | }
   | - `b` dropped here while still borrowed
