
error[E0277]: the trait bound `S: Tr` is not satisfied
  --> src/main.rs:11:9
   |
7  | fn foo<T: Tr>(t: T) {}
   |           -- required by this bound in `foo`
...
11 |     foo(s);
   |         ^
   |         |
   |         expected an implementor of trait `Tr`
   |         help: consider borrowing here: `&s`

error: aborting due to previous error
