
error[E0623]: lifetime mismatch
 --> src/main.rs:9:28
  |
8 | fn mut_ref<'a, 'b>(val: &'a mut &'b mut Foo)  {
  |                         -------------------
  |                         |
  |                         these two types are declared with different lifetimes...
9 |     let tmp: &'b mut Foo = *val;
  |                            ^^^^ ...but data from `val` flows into `val` here

error: aborting due to previous error
