
error: lifetime may not live long enough
 --> src/main.rs:5:12
  |
4 | fn call2<'a, 'b: 'a>(a: &'a usize, b: &'b usize) {
  |          --  -- lifetime `'b` defined here
  |          |
  |          lifetime `'a` defined here
5 |     let z: Option<&'b &'a usize> = None;//~ ERROR E0623
  |            ^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'b`
  |
  = help: consider adding the following bound: `'a: 'b`

error: lifetime may not live long enough
  --> src/main.rs:10:12
   |
8  | fn call3<'a, 'b>(a: &'a usize, b: &'b usize) {
   |          --  -- lifetime `'b` defined here
   |          |
   |          lifetime `'a` defined here
9  |     let y: Paramd<'a> = Paramd { x: a };
10 |     let z: Option<&'b Paramd<'a>> = None;//~ ERROR E0623
   |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`

error: lifetime may not live long enough
  --> src/main.rs:14:12
   |
13 | fn call4<'a, 'b>(a: &'a usize, b: &'b usize) {
   |          --  -- lifetime `'b` defined here
   |          |
   |          lifetime `'a` defined here
14 |     let z: Option<&'a &'b usize> = None;//~ ERROR E0623
   |            ^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
