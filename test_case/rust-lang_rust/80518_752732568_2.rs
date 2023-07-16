
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
  --> src\lib.rs:32:57
   |
32 |     pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item=u8> + 'b
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: hidden type `Map<<Data as IntoIterator>::IntoIter, [closure@src\lib.rs:39:30: 39:88]>` captures the lifetime `'a` as defined on the impl at 18:6
  --> src\lib.rs:18:6
   |
18 | impl<'a> Xorcism<'a> {
   |      ^^

error: aborting due to previous error
