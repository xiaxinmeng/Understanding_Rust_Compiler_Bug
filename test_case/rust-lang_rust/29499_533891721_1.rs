text
error[E0277]: the trait bound `(_, _): NotSame` is not satisfied
  --> src/main.rs:12:5
   |
6  | fn f<T, U>(_: T, _: U) where (T, U): NotSame {}
   | -------------------------------------------- required by `f`
...
12 |     f(S, Z); // error: the trait `NotSame` is not implemented for the type `(_, _)`
   |     ^ the trait `NotSame` is not implemented for `(_, _)`
   |
   = help: the following implementations were found:
             <(A, A) as NotSame>

error[E0277]: the trait bound `(_, _): NotSame` is not satisfied
  --> src/main.rs:13:5
   |
6  | fn f<T, U>(_: T, _: U) where (T, U): NotSame {}
   | -------------------------------------------- required by `f`
...
13 |     f(S, S); // error: the trait `NotSame` is not implemented for the type `(_, _)`
   |     ^ the trait `NotSame` is not implemented for `(_, _)`
   |
   = help: the following implementations were found:
             <(A, A) as NotSame>

error: aborting due to 2 previous errors
