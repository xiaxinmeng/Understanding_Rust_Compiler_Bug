
error: lifetime may not live long enough
  --> src/lib.rs:12:36
   |
6  | fn foo<'a>(&self, x: &'a i32) -> &i32 {
   |        --  - let's call the lifetime of this reference `'1`
   |        |
   |        lifetime `'a` defined here
...
12 |     if true { &self.field } else { x }
   |                                    ^ associated function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'a`

error: lifetime may not live long enough
  --> src/lib.rs:19:5
   |
15 |   fn bar<'a>(&self, x: &'a i32) -> &i32 {
   |          --  - let's call the lifetime of this reference `'1`
   |          |
   |          lifetime `'a` defined here
...
19 |     x
   |     ^ associated function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'a`
