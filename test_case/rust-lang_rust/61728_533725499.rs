
error: lifetime may not live long enough
  --> src/main.rs:14:11
   |
13 | fn f(v: &mut V) {
   |      -  - let's call the lifetime of this reference `'1`
   |      |
   |      has type `&mut V<'2>`
14 |     match v.v.last().unwrap() {
   |           ^^^^^^^^^^ argument requires that `'1` must outlive `'2`

error[E0502]: cannot borrow `v.v` as mutable because it is also borrowed as immutable
  --> src/main.rs:15:28
   |
13 | fn f(v: &mut V) {
   |      - has type `&mut V<'1>`
14 |     match v.v.last().unwrap() {
   |           ----------
   |           |
   |           immutable borrow occurs here
   |           argument requires that `v.v` is borrowed for `'1`
15 |         IntOrRef::I(i) => {v.v.push(IntOrRef::R(&i));}
   |                            ^^^ mutable borrow occurs here
