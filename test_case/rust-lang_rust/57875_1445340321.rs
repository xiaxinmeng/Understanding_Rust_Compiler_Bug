
error[E0308]: mismatched types
  --> src/main.rs:8:15
   |
8  |       let foo = Foo {
   |  _______________^
9  | |         value: 5 as i32,
10 | |         func: lambda
11 | |     };
   | |_____^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> Fn<(&'a i32, &'b i32)>`
              found trait `Fn<(&i32, &i32)>`
note: this closure does not fulfill the lifetime requirements
  --> src/main.rs:7:18
   |
7  |     let lambda = |&x, &y| x + y;
   |                  ^^^^^^^^
note: the lifetime requirement is introduced here
  --> src/main.rs:1:18
   |
1  | struct Foo<T, F: Fn(&T, &T) -> T> {
   |                  ^^^^^^^^^^^^^^^

error: implementation of `FnOnce` is not general enough
  --> src/main.rs:8:15
   |
8  |       let foo = Foo {
   |  _______________^
9  | |         value: 5 as i32,
10 | |         func: lambda
11 | |     };
   | |_____^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'2 i32, &i32) -> i32` must implement `FnOnce<(&'1 i32, &i32)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 i32, &i32)>`, for some specific lifetime `'2`

error: implementation of `FnOnce` is not general enough
  --> src/main.rs:8:15
   |
8  |       let foo = Foo {
   |  _______________^
9  | |         value: 5 as i32,
10 | |         func: lambda
11 | |     };
   | |_____^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&i32, &'2 i32) -> i32` must implement `FnOnce<(&i32, &'1 i32)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&i32, &'2 i32)>`, for some specific lifetime `'2`
