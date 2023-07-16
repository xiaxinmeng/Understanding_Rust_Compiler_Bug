text
error[E0308]: mismatched types
  --> src/main.rs:20:45
   |
20 |     let _: for<'a> fn(&'a str) -> Str<'a> = Str;
   |                                             ^^^ expected concrete lifetime, found bound lifetime parameter 'a
   |
   = note: expected type `for<'a> fn(&'a str) -> Str<'a>`
              found type `fn(&str) -> Str<'_> {Str::<'_>}`

error[E0631]: type mismatch in function arguments
  --> src/main.rs:25:10
   |
3  | fn test<F: for<'x> FnOnce<(&'x str,)>>(_: F) {}
   | -------------------------------------------- required by `test`
...
14 | struct Str<'a>(&'a str);
   | ------------------------ found signature of `fn(&str) -> _`
...
25 |     test(Str);
   |          ^^^ expected signature of `for<'x> fn(&'x str) -> _`

error: aborting due to 2 previous errors
