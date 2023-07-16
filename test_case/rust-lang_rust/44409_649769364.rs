
error[E0491]: in type `&'b &'a usize`, reference has a longer lifetime than the data it references
 --> src/main.rs:4:12
  |
4 |     let z: Option<&'b &'a usize> = None;//~ ERROR E0623
  |            ^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime `'b` as defined on the function body at 3:14
 --> src/main.rs:3:14
  |
3 | fn call2<'a, 'b: 'a>(a: &'a usize, b: &'b usize) {
  |              ^^
note: but the referenced data is only valid for the lifetime `'a` as defined on the function body at 3:10
 --> src/main.rs:3:10
  |
3 | fn call2<'a, 'b: 'a>(a: &'a usize, b: &'b usize) {
  |          ^^

error[E0491]: in type `&'b Paramd<'a>`, reference has a longer lifetime than the data it references
 --> src/main.rs:9:12
  |
9 |     let z: Option<&'b Paramd<'a>> = None;//~ ERROR E0623
  |            ^^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime `'b` as defined on the function body at 7:14
 --> src/main.rs:7:14
  |
7 | fn call3<'a, 'b>(a: &'a usize, b: &'b usize) {
  |              ^^
note: but the referenced data is only valid for the lifetime `'a` as defined on the function body at 7:10
 --> src/main.rs:7:10
  |
7 | fn call3<'a, 'b>(a: &'a usize, b: &'b usize) {
  |          ^^

error[E0491]: in type `&'a &'b usize`, reference has a longer lifetime than the data it references
  --> src/main.rs:13:12
   |
13 |     let z: Option<&'a &'b usize> = None;//~ ERROR E0623
   |            ^^^^^^^^^^^^^^^^^^^^^
   |
note: the pointer is valid for the lifetime `'a` as defined on the function body at 12:10
  --> src/main.rs:12:10
   |
12 | fn call4<'a, 'b>(a: &'a usize, b: &'b usize) {
   |          ^^
note: but the referenced data is only valid for the lifetime `'b` as defined on the function body at 12:14
  --> src/main.rs:12:14
   |
12 | fn call4<'a, 'b>(a: &'a usize, b: &'b usize) {
   |              ^^
