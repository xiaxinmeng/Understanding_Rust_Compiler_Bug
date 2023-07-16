
error[E0107]: wrong number of type arguments: expected 1, found 2
 --> src/main.rs:8:16
  |
8 | impl Fn<isize, isize> for Foo {
  |                ^^^^^ unexpected type argument

error[E0107]: wrong number of type arguments: expected 1, found 2
  --> src/main.rs:13:19
   |
13 | impl FnMut<isize, isize> for Foo {
   |                   ^^^^^ unexpected type argument

error[E0107]: wrong number of type arguments: expected 1, found 2
  --> src/main.rs:18:20
   |
18 | impl FnOnce<isize, isize> for Foo {
   |                    ^^^^^ unexpected type argument
