
error[E0308]: mismatched types
  --> src/main.rs:30:5
   |
30 |     foo2(|x,y| ()); // Error, but works with one argument
   |     ^^^^ lifetime mismatch
   |
   = note: expected type `for<'r, 's> Fn<(&'r str, &'s str)>`
              found type `Fn<(&str, &str)>`
note: this closure does not fulfill the lifetime requirements
  --> src/main.rs:30:10
   |
30 |     foo2(|x,y| ()); // Error, but works with one argument
   |          ^^^^^^^^
note: the lifetime requirement is introduced here
  --> src/main.rs:15:12
   |
15 | fn foo2<F: Foo>(f: F) {
   |            ^^^

error: implementation of `Fn` is not general enough
  --> src/main.rs:31:10
   |
31 |     foo3(Box::new(&|x,y| ())); // Error, even with one argument
   |          ^^^^^^^^^^^^^^^^^^^ implementation of `Fn` is not general enough
   |
   = note: closure with signature `fn(&'2 str, &str)` must implement `Fn<(&'1 str, &str)>`, for any lifetime `'1`...
   = note: ...but it actually implements `Fn<(&'2 str, &str)>`, for some specific lifetime `'2`

error: implementation of `Fn` is not general enough
  --> src/main.rs:31:10
   |
31 |     foo3(Box::new(&|x,y| ())); // Error, even with one argument
   |          ^^^^^^^^^^^^^^^^^^^ implementation of `Fn` is not general enough
   |
   = note: closure with signature `fn(&str, &'2 str)` must implement `Fn<(&str, &'1 str)>`, for any lifetime `'1`...
   = note: ...but it actually implements `Fn<(&str, &'2 str)>`, for some specific lifetime `'2`
