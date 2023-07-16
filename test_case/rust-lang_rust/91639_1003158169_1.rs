
error[E0623]: lifetime mismatch
 --> src/lib.rs:8:9
  |
7 | fn bar(foo: &mut Foo) {
  |             --------
  |             |
  |             these two types are declared with different lifetimes...
8 |     foo.modify();
  |         ^^^^^^ ...but data from `foo` flows into `foo` here
