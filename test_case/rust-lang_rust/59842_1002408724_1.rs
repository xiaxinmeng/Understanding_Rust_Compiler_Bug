
error[E0623]: lifetime mismatch
  --> src/lib.rs:17:26
   |
16 | fn func<'a, 'b, B>(foo: Foo<'a, Bar<'b, B>>) {
   |                         ------------------- these two types are declared with different lifetimes...
17 |     let _: &Bar<'b, B> = foo.inner();
   |                          ^^^^^^^^^^^ ...but data from `foo` flows into `foo` here
