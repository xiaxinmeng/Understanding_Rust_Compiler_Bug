
error: unsatisfied lifetime constraints
  --> src/lib.rs:10:15
   |
8  | fn foo<'a>(&self, x: &'a Foo) -> &'a Foo {
   |        --  - let's call the lifetime of this reference `'1`
   |        |
   |        lifetime `'a` defined here
9  | 
10 |     if true { self } else { x }
   |               ^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
